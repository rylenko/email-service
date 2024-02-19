use anyhow::{Context as _, Result};

pub(crate) struct Db(common::helpers::DbPool);

impl Db {
	pub(crate) async fn connect() -> Result<Self> {
		let pool = common::helpers::create_db_pool()
			.context("Failed to create a db pool.")?;
		// TODO: without this connection will not be accepted...
		let _e = pool.get().await.unwrap();
		Ok(Self(pool))
	}

	fn decrypt_email(
		email: &crate::models::Email,
		cipher: &common::crypto::AesCipher,
	) -> Result<crate::raw_models::Email> {
		let sender_public_key = cipher
			.decrypt_base64(&email.encrypted_sender_public_key_pem)
			.context("Failed to decrypt base64 sender public key.")?;
		let data_bytes = cipher
			.decrypt(&email.encrypted_data_bytes)
			.context("Failed to decrypt data bytes.")?;
		let data = bincode::deserialize(&data_bytes)
			.context("Failed to deserialize data bytes.")?;
		Ok(crate::raw_models::Email::new(email.id, sender_public_key, data))
	}

	fn decrypt_friend(
		friend: &crate::models::Friend,
		cipher: &common::crypto::AesCipher,
	) -> Result<crate::raw_models::Friend> {
		let username = cipher
			.decrypt_string(&friend.encrypted_username)
			.context("Failed to decrypt a username.")?;
		let public_key = cipher
			.decrypt_string(&friend.encrypted_public_key_pem_base64)
			.context("Failed to decrypt a public key.")?;
		Ok(crate::raw_models::Friend::new(friend.id, username, public_key))
	}

	pub(crate) async fn get_user(
		&self,
		username: String,
		password: String,
	) -> Result<crate::raw_models::User> {
		use {
			crate::schema::users::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Find user by username
		let username_hash = common::crypto::hash(&username).to_vec();
		let mut connection = self.0.get().await?;
		let db_user: crate::models::User = table
			.filter(dsl::username_hash.eq(username_hash))
			.first(&mut connection)
			.await?;

		// Check the password
		let password_hash =
			common::crypto::hash_with_salt(&password, &db_user.salt);
		if password_hash[..] != db_user.password_hash {
			return Err(diesel::result::Error::NotFound.into());
		}
		Ok(crate::raw_models::User::new(db_user.id, username, password))
	}

	pub(crate) async fn get_user_private_key(
		&self,
		user: &crate::raw_models::User,
	) -> Result<openssl::rsa::Rsa<openssl::pkey::Private>> {
		use {
			crate::schema::users::{dsl, table},
			diesel::QueryDsl as _,
			diesel_async::RunQueryDsl as _,
		};

		// Get an encrypted private key
		let mut connection = self.0.get().await?;
		let query =
			table.find(user.id()).select(dsl::encrypted_private_key_pem);
		let encrypted_private_key: Vec<u8> =
			query.first(&mut connection).await?;

		// Make a cipher and decrypt the private key
		let pem = user.make_aes_cipher().decrypt(&encrypted_private_key)?;
		let private_key = openssl::rsa::Rsa::private_key_from_pem(&pem)?;
		Ok(private_key)
	}

	pub(crate) async fn get_user_salt(
		&self,
		user: &crate::raw_models::User,
	) -> Result<Vec<u8>> {
		use {
			crate::schema::users::{dsl, table},
			diesel::QueryDsl as _,
			diesel_async::RunQueryDsl as _,
		};
		let mut connection = self.0.get().await?;
		let salt = table
			.find(user.id())
			.select(dsl::salt)
			.first(&mut connection)
			.await?;
		Ok(salt)
	}

	/// # Panic
	///
	/// If `current_page` equal to `0`.
	pub(crate) async fn get_emails(
		&self,
		user: &crate::raw_models::User,
		current_page: std::num::NonZeroU64,
	) -> Result<crate::app::pagination::Pagination<crate::raw_models::Email>> {
		use {
			crate::schema::emails::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		let mut connection = self.0.get().await?;
		let user_filter = table.filter(dsl::user_id.eq(user.id()));

		// Get database emails
		let offset =
			(u64::from(current_page) - 1) * (crate::consts::EMAILS_PER_PAGE);
		#[allow(clippy::cast_possible_wrap)]
		let db_emails = user_filter
			.offset(offset as i64)
			.limit(crate::consts::EMAILS_PER_PAGE as i64)
			.order(dsl::created_at.desc())
			.load::<crate::models::Email>(&mut connection)
			.await?;

		// Make a cipher and decrypt database emails
		let cipher = user.make_aes_cipher();
		let mut raw_emails = Vec::with_capacity(db_emails.len());
		for db_email in db_emails {
			raw_emails.push(Self::decrypt_email(&db_email, &cipher)?);
		}

		// Get pages count and make pagination
		let count: i64 =
			user_filter.count().get_result(&mut connection).await?;
		#[allow(clippy::cast_sign_loss)]
		let pages = (count as u64).div_ceil(crate::consts::EMAILS_PER_PAGE);
		Ok(crate::app::pagination::Pagination::new(
			current_page,
			pages,
			raw_emails,
		))
	}

	pub(crate) async fn get_email(
		&self,
		user: &crate::raw_models::User,
		id: i32,
	) -> Result<crate::raw_models::Email> {
		use {
			crate::schema::emails::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Get database email
		let mut connection = self.0.get().await?;
		let db_email = table
			.filter(dsl::user_id.eq(user.id()))
			.find(id)
			.first(&mut connection)
			.await?;
		// Make a cipher and decrypt the database email
		let cipher = user.make_aes_cipher();
		let raw_email = Self::decrypt_email(&db_email, &cipher)?;
		Ok(raw_email)
	}

	pub(crate) async fn check_user_f2f(
		&self,
		user: &crate::raw_models::User,
	) -> Result<bool> {
		use {
			crate::schema::users::{dsl, table},
			diesel::QueryDsl as _,
			diesel_async::RunQueryDsl as _,
		};

		// Make and execute the query
		let mut connection = self.0.get().await?;
		let query = table.find(user.id()).select(dsl::f2f_enabled);
		let is_enabled = query.first(&mut connection).await?;
		Ok(is_enabled)
	}

	pub(crate) async fn check_user_exists(
		&self,
		username: &str,
	) -> Result<bool> {
		use {
			crate::schema::users::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};
		let mut connection = self.0.get().await?;
		let username_hash = common::crypto::hash(username).to_vec();
		let filter = table.filter(dsl::username_hash.eq(username_hash));
		let exists = diesel::select(diesel::dsl::exists(filter))
			.get_result(&mut connection)
			.await?;
		Ok(exists)
	}

	pub(crate) async fn create_user(
		&self,
		username: &str,
		password: &str,
		private_key: &openssl::rsa::Rsa<openssl::pkey::Private>,
	) -> Result<()> {
		use {crate::schema::users::table, diesel_async::RunQueryDsl as _};
		debug_assert!(!self.check_user_exists(username).await?);

		let new_user =
			crate::models::NewUser::new(username, password, private_key)?;
		let mut connection = self.0.get().await?;
		diesel::insert_into(table)
			.values(new_user)
			.execute(&mut connection)
			.await?;
		Ok(())
	}

	pub(crate) async fn switch_user_f2f(
		&self,
		user: &crate::raw_models::User,
	) -> Result<bool> {
		use {
			crate::schema::users::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		let new_value = !self.check_user_f2f(user).await?;
		let mut connection = self.0.get().await?;
		diesel::update(table.find(user.id()))
			.set(dsl::f2f_enabled.eq(new_value))
			.execute(&mut connection)
			.await?;
		Ok(new_value)
	}

	pub(crate) async fn delete_user(
		&self,
		user: &crate::raw_models::User,
	) -> Result<()> {
		use {
			crate::schema::users::table, diesel::QueryDsl as _,
			diesel_async::RunQueryDsl as _,
		};

		let mut connection = self.0.get().await?;
		diesel::delete(table.find(user.id())).execute(&mut connection).await?;
		Ok(())
	}

	pub(crate) async fn check_friend_exists_by_username(
		&self,
		user: &crate::raw_models::User,
		username: &str,
	) -> Result<bool> {
		use {
			crate::schema::friends::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Hash the username
		let salt = self.get_user_salt(user).await?;
		let username_hash =
			common::crypto::hash_with_salt(username, &salt).to_vec();

		// Make and execute the query
		let mut connection = self.0.get().await?;
		let filter = table
			.filter(dsl::user_id.eq(user.id()))
			.filter(dsl::username_hash.eq(username_hash));
		let exists = diesel::select(diesel::dsl::exists(filter))
			.get_result(&mut connection)
			.await?;
		Ok(exists)
	}

	pub(crate) async fn get_friends(
		&self,
		user: &crate::raw_models::User,
	) -> Result<Vec<crate::raw_models::Friend>> {
		use {
			crate::schema::friends::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Get database friends
		let mut connection = self.0.get().await?;
		let db_friends = table
			.filter(dsl::user_id.eq(user.id()))
			.load::<crate::models::Friend>(&mut connection)
			.await?;

		// Make a cipher and decrypt database friends
		let cipher = user.make_aes_cipher();
		let mut raw_friends = Vec::with_capacity(db_friends.len());
		for db_friend in db_friends {
			raw_friends.push(Self::decrypt_friend(&db_friend, &cipher)?);
		}

		// Sort by username and return
		raw_friends.sort_by(|f1, f2| f1.username().cmp(f2.username()));
		Ok(raw_friends)
	}

	pub(crate) async fn get_friend(
		&self,
		user: &crate::raw_models::User,
		public_key_pem_base64: &str,
	) -> Result<crate::raw_models::Friend> {
		use {
			crate::schema::friends::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Hash public key
		let salt = self.get_user_salt(user).await?;
		let public_key_pem_base64_hash =
			common::crypto::hash_with_salt(public_key_pem_base64, &salt)
				.to_vec();

		// Get database friend
		let mut connection = self.0.get().await?;
		let db_friend = table
			.filter(dsl::user_id.eq(user.id()))
			.filter(
				dsl::public_key_pem_base64_hash.eq(public_key_pem_base64_hash),
			)
			.first(&mut connection)
			.await?;

		// Make a cipher and decrypt the database friend
		let cipher = user.make_aes_cipher();
		let raw_friend = Self::decrypt_friend(&db_friend, &cipher)?;
		Ok(raw_friend)
	}

	pub(crate) async fn get_nodes(
		&self,
		user: &crate::raw_models::User,
	) -> Result<Vec<crate::raw_models::Node>> {
		use {
			crate::schema::nodes::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Get database nodes
		let mut connection = self.0.get().await?;
		let db_nodes = table
			.filter(dsl::user_id.eq(user.id()))
			.order(dsl::created_at.desc())
			.load::<crate::models::Node>(&mut connection)
			.await?;
		// Make a cipher and decrypt database nodes
		let cipher = user.make_aes_cipher();
		let mut raw_nodes = Vec::with_capacity(db_nodes.len());
		for db_node in db_nodes {
			// Decrypt data
			let address =
				cipher.decrypt_string(&db_node.encrypted_address)?.parse()?;
			let password = match db_node.encrypted_password {
				Some(ep) => Some(cipher.decrypt_string(&ep)?),
				None => None,
			};
			raw_nodes.push(crate::raw_models::Node::new(
				db_node.id, address, password,
			));
		}
		Ok(raw_nodes)
	}

	pub(crate) async fn check_friend_exists_by_public_key(
		&self,
		user: &crate::raw_models::User,
		public_key_pem_base64: &str,
	) -> Result<bool> {
		use {
			crate::schema::friends::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Hash the public key
		let salt = self.get_user_salt(user).await?;
		let public_key_pem_base64_hash =
			common::crypto::hash_with_salt(public_key_pem_base64, &salt)
				.to_vec();

		// Make and execute the query
		let filter = table.filter(dsl::user_id.eq(user.id())).filter(
			dsl::public_key_pem_base64_hash.eq(public_key_pem_base64_hash),
		);
		let mut connection = self.0.get().await?;
		let exists = diesel::select(diesel::dsl::exists(filter))
			.get_result(&mut connection)
			.await?;
		Ok(exists)
	}

	pub(crate) async fn add_friend(
		&self,
		user: &crate::raw_models::User,
		username: &str,
		public_key_pem_base64: &str,
	) -> Result<()> {
		use {crate::schema::friends::table, diesel_async::RunQueryDsl as _};
		debug_assert!(
			!self.check_friend_exists_by_username(user, username).await?
		);
		debug_assert!(
			!self
				.check_friend_exists_by_public_key(user, public_key_pem_base64)
				.await?
		);

		let new_friend = crate::models::NewFriend::new(
			self,
			user,
			username,
			public_key_pem_base64,
		)
		.await?;
		let mut connection = self.0.get().await?;
		diesel::insert_into(table)
			.values(new_friend)
			.execute(&mut connection)
			.await?;
		Ok(())
	}

	pub(crate) async fn delete_friend(
		&self,
		user: &crate::raw_models::User,
		id: i32,
	) -> Result<()> {
		use {
			crate::schema::friends::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Find and delete
		let mut connection = self.0.get().await?;
		let deleted_rows = diesel::delete(table.find(id))
			.filter(dsl::user_id.eq(user.id()))
			.execute(&mut connection)
			.await?;
		if deleted_rows == 0 {
			return Err(diesel::result::Error::NotFound.into());
		}
		Ok(())
	}

	pub(crate) async fn check_email_exists(
		&self,
		user: &crate::raw_models::User,
		email: &common::email::Email,
	) -> Result<bool> {
		use {
			crate::schema::emails::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Make and execute the query
		let mut connection = self.0.get().await?;
		let filter = table
			.filter(dsl::user_id.eq(user.id()))
			.filter(dsl::proof_of_work.eq(email.compute_hash()));
		let exists = diesel::select(diesel::dsl::exists(filter))
			.get_result(&mut connection)
			.await?;
		Ok(exists)
	}

	/// # Debug panic
	///
	/// If you have not used `email.decrypt` or
	/// `email.check_decrypted_integrity()` is `false`.
	pub(crate) async fn add_email(
		&self,
		user: &crate::raw_models::User,
		email: &common::email::Email,
	) -> Result<()> {
		use {crate::schema::emails::table, diesel_async::RunQueryDsl as _};
		debug_assert!(email.check_decrypted_integrity()?);
		debug_assert!(!self.check_email_exists(user, email).await?);

		let new_email = crate::models::NewEmail::new(user, email)?;
		let mut connection = self.0.get().await?;
		diesel::insert_into(table)
			.values(new_email)
			.execute(&mut connection)
			.await?;
		Ok(())
	}

	pub(crate) async fn check_node_exists(
		&self,
		user: &crate::raw_models::User,
		address: &str,
	) -> Result<bool> {
		use {
			crate::schema::nodes::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		// Hash the address with salt
		let salt = self.get_user_salt(user).await?;
		let address_hash = common::crypto::hash_with_salt(address, &salt);

		// Make and execute the query
		let mut connection = self.0.get().await?;
		let filter = table
			.filter(dsl::user_id.eq(user.id()))
			.filter(dsl::address_hash.eq(address_hash.to_vec()));
		let exists = diesel::select(diesel::dsl::exists(filter))
			.get_result(&mut connection)
			.await?;
		Ok(exists)
	}

	pub(crate) async fn add_node(
		&self,
		user: &crate::raw_models::User,
		host: &str,
		password: Option<&str>,
	) -> Result<()> {
		use {crate::schema::nodes::table, diesel_async::RunQueryDsl as _};
		debug_assert!(!self.check_node_exists(user, host).await?);

		let new_node =
			crate::models::NewNode::new(self, user, host, password).await?;
		let mut connection = self.0.get().await?;
		diesel::insert_into(table)
			.values(new_node)
			.execute(&mut connection)
			.await?;
		Ok(())
	}

	pub(crate) async fn delete_node(
		&self,
		user: &crate::raw_models::User,
		id: i32,
	) -> Result<()> {
		use {
			crate::schema::nodes::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		let mut connection = self.0.get().await?;
		let deleted_rows = diesel::delete(table.find(id))
			.filter(dsl::user_id.eq(user.id()))
			.execute(&mut connection)
			.await?;
		if deleted_rows == 0 {
			return Err(diesel::result::Error::NotFound.into());
		}
		Ok(())
	}

	pub(crate) async fn delete_old_emails(
		&self,
		than: std::time::Duration,
	) -> Result<()> {
		use {
			crate::schema::emails::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		let mut connection = self.0.get().await?;
		let filter = table
			.filter(dsl::created_at.lt(std::time::SystemTime::now() - than));
		diesel::delete(filter).execute(&mut connection).await?;
		Ok(())
	}
}
