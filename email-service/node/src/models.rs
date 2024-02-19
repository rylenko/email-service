use anyhow::{Context as _, Error, Result};

/// # Explanation of some fields
///
/// `self.recipient_public_key_pem_hash` - Duplicate of field from `Email`.
/// `self.proof_of_work` - proof of work from `Email`. Needed to avoid
/// duplicates.
#[derive(diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::emails)]
pub(super) struct NewEmail {
	email_bytes: Vec<u8>,
	recipient_public_key_pem_hash: Vec<u8>,
	proof_of_work: String,
}

impl std::convert::TryFrom<&common::email::Email> for NewEmail {
	type Error = Error;

	/// # Debug panic
	///
	/// If `email.check_encrypted_integrity()` is `false`.
	fn try_from(email: &common::email::Email) -> Result<Self> {
		debug_assert!(email.check_encrypted_integrity());

		let email_bytes =
			bincode::serialize(email).context("Failed to serialize email.")?;
		Ok(Self {
			email_bytes,
			recipient_public_key_pem_hash: email
				.recipient_public_key_pem_hash()
				.to_vec(),
			proof_of_work: email.compute_hash(),
		})
	}
}
