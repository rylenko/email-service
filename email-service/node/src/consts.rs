lazy_static::lazy_static! {
	pub(crate) static ref CONFIG_PATH: std::path::PathBuf =
		std::env::var("CONFIG_PATH").unwrap().into();
}

pub(crate) const CONTAINER_ADDRESS: &str = "0.0.0.0:8000";
