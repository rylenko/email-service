lazy_static::lazy_static! {
	pub static ref DEBUG: bool = std::env::var("DEBUG").is_ok();
	pub static ref DB_URL: String = std::env::var("DATABASE_URL").unwrap();

	pub(crate) static ref PROOF_OF_WORK_DIFFICULTY_STRING: String =
		"0".repeat(PROOF_OF_WORK_DIFFICULTY as usize);
}

pub(crate) const PACKAGE_MAX_SIZE: usize = 10 * 1024 * 1024; // 10 MiB
pub(crate) const PACKAGE_RECEIVE_TIMEOUT: std::time::Duration =
	std::time::Duration::from_secs(5);
pub(crate) const PROOF_OF_WORK_DIFFICULTY: u8 = 5;
pub(crate) const DEFAULT_RANDOM_BYTES_LENGTH: usize = 32;

pub const EMAILS_MAX_AGE: std::time::Duration =
	std::time::Duration::from_secs(86400 * 2); // 2 days
pub const CHECK_OLD_EMAILS_INTERVAL: std::time::Duration =
	std::time::Duration::from_secs(86400); // 1 day
pub const PASSWORD_SALT: &[u8; 13] = b"password-salt";
