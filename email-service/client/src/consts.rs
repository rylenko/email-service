lazy_static::lazy_static! {
	static ref BASE_DIR: std::path::PathBuf = env!("CARGO_MANIFEST_DIR").into();
	pub(crate) static ref STATIC_DIR: std::path::PathBuf
		= BASE_DIR.join("static");
	pub(crate) static ref CONFIG_PATH: std::path::PathBuf
		= std::env::var("CONFIG_PATH").unwrap().into();
}

pub(crate) const CONTAINER_ADDRESS: &str = "0.0.0.0:8000";
pub(crate) const CSRF_COOKIE_NAME: &str = "csrf-token";

pub(crate) const EMAILS_PER_PAGE: u64 = 4;
common::const_assert!(EMAILS_PER_PAGE < i64::MAX as u64);

pub(crate) const NEW_EMAILS_FROM_NODE_LIMIT: u8 = 4;
pub(crate) const RSA_KEY_SIZE: u32 = 2048;

pub(crate) const TERA_DIR_STR: &str =
	concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*");
