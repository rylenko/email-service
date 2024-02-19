use anyhow::{Context as _, Result};

/// # Explanation of some fields
///
/// aes key = sha256(user password, user username)
///
/// `self.username_hash` = sha256(user username)
/// `self.password_hash` = sha256(password, user salt)
/// `self.encrypted_private_key_pem` = aes[aes key](private key pem)
#[allow(dead_code)]
#[derive(diesel::prelude::Queryable)]
pub(crate) struct User {
	pub id: i32,
	pub username_hash: Vec<u8>,
	pub password_hash: Vec<u8>,
	pub encrypted_private_key_pem: Vec<u8>,
	pub salt: Vec<u8>,
	pub f2f_enabled: bool,
	pub created_at: chrono::NaiveDateTime,
}

/// Used to create a new user. For more information see `User`.
///
/// See also `User`.
#[derive(diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::users)]
pub(crate) struct NewUser {
	username_hash: Vec<u8>,
	password_hash: Vec<u8>,
	encrypted_private_key_pem: Vec<u8>,
	salt: Vec<u8>,
	f2f_enabled: bool,
}

impl NewUser {
	pub fn new(
		username: &str,
		password: &str,
		private_key: &openssl::rsa::Rsa<openssl::pkey::Private>,
	) -> Result<Self> {
		// Hash username and password
		let salt = common::crypto::generate_random_bytes(None)
			.context("Failed to generate a salt.")?;
		let username_hash = common::crypto::hash(username);
		let password_hash = common::crypto::hash_with_salt(password, &salt);

		// Make cipher and encrypt private key
		let key = common::crypto::hash_with_salt(password, username);
		let cipher = common::crypto::AesCipher::new(&key[..]);
		let private_key_pem = private_key
			.private_key_to_pem()
			.context("Failed to get private key pem.")?;
		let encrypted_private_key_pem = cipher
			.encrypt(private_key_pem)
			.context("Failed to encrypt private key pem.")?;

		Ok(Self {
			username_hash: username_hash.to_vec(),
			password_hash: password_hash.to_vec(),
			encrypted_private_key_pem,
			salt,
			f2f_enabled: false,
		})
	}
}

/// # Explanation of some fields
///
/// aes key = sha256(current user password, current user username)
///
/// `self.username_hash` = sha256(friend username, current user salt)
/// `self.public_key_pem_base64_hash` = sha256(base64(friend public key pem),
/// current user salt)
/// `self.encrypted_username` = aes[aes key](friend_ username)
/// `self.encrypted_public_key_pem_base64` =
/// aes[aes key](base64(friend public key pem))
#[allow(dead_code)]
#[derive(diesel::prelude::Queryable)]
pub(crate) struct Friend {
	pub id: i32,
	pub user_id: i32,
	pub username_hash: Vec<u8>,
	pub public_key_pem_base64_hash: Vec<u8>,
	pub encrypted_username: Vec<u8>,
	pub encrypted_public_key_pem_base64: Vec<u8>,
	pub created_at: chrono::NaiveDateTime,
}

/// Used to add a new friend. For more information see `Friend`.
///
/// See also `Friend`.
#[derive(diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::friends)]
pub(crate) struct NewFriend {
	user_id: i32,
	username_hash: Vec<u8>,
	public_key_pem_base64_hash: Vec<u8>,
	encrypted_username: Vec<u8>,
	encrypted_public_key_pem_base64: Vec<u8>,
}

impl NewFriend {
	pub async fn new(
		db: &crate::db::Db,
		user: &crate::raw_models::User,
		username: &str,
		public_key_pem_base64: &str,
	) -> Result<Self> {
		// Hash the username
		let salt = db
			.get_user_salt(user)
			.await
			.context("Failed to get user salt.")?;
		let username_hash = common::crypto::hash_with_salt(username, &salt);
		let public_key_pem_base64_hash =
			common::crypto::hash_with_salt(public_key_pem_base64, &salt);

		// Encrypt username and public key pem
		let cipher = user.make_aes_cipher();
		let encrypted_username =
			cipher.encrypt(username).context("Failed to encrypt username.")?;
		let encrypted_public_key_pem_base64 = cipher
			.encrypt(public_key_pem_base64)
			.context("Failed to encrypt public key pem base64.")?;

		Ok(Self {
			user_id: user.id(),
			username_hash: username_hash.into(),
			public_key_pem_base64_hash: public_key_pem_base64_hash.into(),
			encrypted_username,
			encrypted_public_key_pem_base64,
		})
	}
}

/// # Explanation of some fields
///
/// aes key = sha256(current user password, current user username)
///
/// `self.encrypted_sender_public_key_pem` =
/// aes[aes key](sender public key pem)
/// `self.encrypted_data_bytes` = aes[aes key](`common::email::Data` bytes)
/// `self.proof_of_work` = proof of work from `Email`. Needed to avoid
/// duplicates.
#[allow(dead_code)]
#[derive(diesel::prelude::Queryable)]
pub(crate) struct Email {
	pub id: i32,
	pub user_id: i32,
	pub encrypted_sender_public_key_pem: Vec<u8>,
	pub encrypted_data_bytes: Vec<u8>,
	pub proof_of_work: String,
	pub created_at: chrono::NaiveDateTime,
}

/// Used to add a new email. For more information see `Email`.
///
/// See also `Email`.
#[derive(diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::emails)]
pub(crate) struct NewEmail {
	user_id: i32,
	encrypted_sender_public_key_pem: Vec<u8>,
	encrypted_data_bytes: Vec<u8>,
	proof_of_work: String,
}

impl NewEmail {
	/// # Debug panic
	///
	/// If `email.check_decrypted_integrity()` is `false`.
	pub fn new(
		user: &crate::raw_models::User,
		email: &common::email::Email,
	) -> Result<Self> {
		debug_assert!(email
			.check_decrypted_integrity()
			.context("Failed to check decrypted integrity.")?);

		let sender_public_key_pem = email.sender_public_key_pem().unwrap();
		let data_bytes = bincode::serialize(email.data().unwrap())
			.context("Failed to serialize email data.")?;

		let cipher = user.make_aes_cipher();
		let encrypted_sender_public_key_pem = cipher
			.encrypt(sender_public_key_pem)
			.context("Failed to encrypt sender public key pem.")?;
		let encrypted_data_bytes = cipher
			.encrypt(data_bytes)
			.context("Failed to encrypt data bytes.")?;

		Ok(Self {
			user_id: user.id(),
			encrypted_sender_public_key_pem,
			encrypted_data_bytes,
			proof_of_work: email.compute_hash(),
		})
	}
}

/// # Explanation of some fields
///
/// aes key = sha256(current user password, current user username)
///
/// `self.address_hash` = sha256(address, current user salt)
/// `self.encrypted_address` = aes[aes key](address)
/// `self.encrypted_password` = aes[aes key](password)
#[allow(dead_code)]
#[derive(diesel::prelude::Queryable)]
pub(crate) struct Node {
	pub id: i32,
	pub user_id: i32,
	pub address_hash: Vec<u8>,
	pub encrypted_address: Vec<u8>,
	pub encrypted_password: Option<Vec<u8>>,
	pub created_at: chrono::NaiveDateTime,
}

/// Used to add a new node. For more information see `Node`.
///
/// See also `Node`.
#[derive(diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::nodes)]
pub(crate) struct NewNode {
	user_id: i32,
	address_hash: Vec<u8>,
	encrypted_address: Vec<u8>,
	encrypted_password: Option<Vec<u8>>,
}

impl NewNode {
	pub async fn new(
		db: &crate::db::Db,
		user: &crate::raw_models::User,
		address: &str,
		password: Option<&str>,
	) -> Result<Self> {
		let salt = db
			.get_user_salt(user)
			.await
			.context("Failed to get user salt.")?;
		let address_hash = common::crypto::hash_with_salt(address, salt);

		// Encrypt username and public key pem
		let cipher = user.make_aes_cipher();
		let encrypted_address =
			cipher.encrypt(address).context("Failed to encrypt address.")?;
		let encrypted_password = match password {
			Some(p) => Some(
				cipher.encrypt(p).context("Failed to encrypt password.")?,
			),
			None => None,
		};
		Ok(Self {
			user_id: user.id(),
			address_hash: address_hash.into(),
			encrypted_address,
			encrypted_password,
		})
	}
}
