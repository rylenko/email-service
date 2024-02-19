use anyhow::{Context as _, Result};

/// The configuration of [`Client`](client::Client).
#[derive(serde::Deserialize)]
#[non_exhaustive]
pub(crate) struct Config {
	dark_theme: bool,
	proxy: Option<std::net::SocketAddr>,
	secret_key: String,
}

impl Config {
	const SECRET_KEY_MIN_LENGTH: usize = 64;

	common::accessor!(copy dark_theme -> bool);

	common::accessor!(copy proxy -> Option<std::net::SocketAddr>);

	common::accessor!(& secret_key -> &str);

	pub async fn load() -> Result<Self> {
		common::debug!("Loading config...");

		let config: Self = common::helpers::deserialize_json_from_file(
			crate::consts::CONFIG_PATH.as_path(),
		)
		.await
		.context("Failed to deserialize JSON from file.")?;
		// See `actix_web::cookie::Key`
		assert!(
			config.secret_key.len() >= Self::SECRET_KEY_MIN_LENGTH,
			"The length of the secret key must be >= 64.",
		);
		Ok(config)
	}
}
