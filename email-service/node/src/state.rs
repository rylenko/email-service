use anyhow::{Context as _, Result};

pub(crate) struct State {
	config: crate::config::Config,
	db: crate::db::Db,
}

impl State {
	common::accessor!(& config -> &crate::config::Config);

	common::accessor!(& db -> &crate::db::Db);

	pub(crate) async fn try_default() -> Result<Self> {
		Ok(Self {
			config: crate::config::Config::load()
				.await
				.context("Failed to load the config.")?,
			db: crate::db::Db::connect()
				.context("Failed to connect to db.")?,
		})
	}
}
