use anyhow::{Context as _, Result};

pub(crate) struct Db(common::helpers::DbPool);

impl Db {
	pub(crate) fn connect() -> Result<Self> {
		let db_pool = common::helpers::create_db_pool()
			.context("Failed to create a db pool.")?;
		Ok(Self(db_pool))
	}

	pub(crate) async fn get_email_bytes(
		&self,
		index: i64,
		recipient_public_key_hash: &[u8; 32],
	) -> Result<Vec<u8>> {
		use {
			crate::schema::emails::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		let mut connection =
			self.0.get().await.context("Failed to get a connection.")?;
		let bytes: Vec<u8> = table
			.filter(
				dsl::recipient_public_key_pem_hash
					.eq(recipient_public_key_hash.to_vec()),
			)
			.offset(index)
			.select(dsl::email_bytes)
			.first(&mut connection)
			.await
			.context("Failed to execute a query.")?;
		Ok(bytes)
	}

	pub(crate) async fn get_emails_count(
		&self,
		recipient_public_key_hash: &[u8; 32],
	) -> Result<i64> {
		use {
			crate::schema::emails::{dsl, table},
			diesel::{ExpressionMethods as _, QueryDsl as _},
			diesel_async::RunQueryDsl as _,
		};

		let mut connection =
			self.0.get().await.context("Failed to get a connection.")?;
		let count = table
			.filter(
				dsl::recipient_public_key_pem_hash
					.eq(recipient_public_key_hash.to_vec()),
			)
			.count()
			.get_result::<i64>(&mut connection)
			.await
			.context("Failed to execute a query.")?;
		Ok(count)
	}

	/// # Debug panic
	///
	/// If `email.check_encrypted_integrity()` is `false`.
	pub(crate) async fn add_email(
		&self,
		email: &common::email::Email,
	) -> Result<()> {
		use {
			crate::schema::emails::table, diesel_async::RunQueryDsl as _,
			std::convert::TryFrom as _,
		};
		debug_assert!(email.check_encrypted_integrity());

		let new_email = crate::models::NewEmail::try_from(email)
			.context("Failed to create a new email.")?;
		let mut connection =
			self.0.get().await.context("Failed to get a connection.")?;
		diesel::insert_into(table)
			.values(new_email)
			.execute(&mut connection)
			.await
			.context("Failed to execute a query.")?;
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

		let mut connection =
			self.0.get().await.context("Failed to get a connection.")?;
		let filter = table
			.filter(dsl::created_at.lt(std::time::SystemTime::now() - than));
		diesel::delete(filter)
			.execute(&mut connection)
			.await
			.context("Failed to execute a query.")?;
		Ok(())
	}
}
