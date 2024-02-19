use anyhow::{Context as _, Result};

pub(crate) struct State {
	config: crate::config::Config,
	db: crate::db::Db,
	tera: tera::Tera,
}

impl State {
	common::accessor!(& config -> &crate::config::Config);

	common::accessor!(& db -> &crate::db::Db);

	common::accessor!(& tera -> &tera::Tera);

	pub(crate) async fn try_default() -> Result<Self> {
		Ok(Self {
			config: crate::config::Config::load()
				.await
				.context("Failed to load the config.")?,
			db: crate::db::Db::connect()
				.await
				.context("Failed to connect to a db.")?,
			tera: crate::app::tera::make_tera(),
		})
	}
}
