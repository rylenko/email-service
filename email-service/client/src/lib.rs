#![deny(clippy::correctness)]
#![feature(error_iter, int_roundings)]
#![warn(
	clippy::complexity,
	clippy::pedantic,
	clippy::perf,
	clippy::style,
	clippy::suspicious
)]
#![allow(
	clippy::as_conversions,
	clippy::implicit_return,
	clippy::missing_docs_in_private_items,
	clippy::missing_errors_doc
)]

mod app;
mod config;
mod consts;
mod db;
mod models;
mod raw_models;
mod schema;
mod state;
mod task;

use anyhow::{Context as _, Result};

pub async fn launch() -> Result<()> {
	let state = actix_web::web::Data::new(
		state::State::try_default()
			.await
			.context("Failed to make a new default state.")?,
	);
	tokio::spawn(task::delete_old_emails_task(state.clone()));

	common::debug!(
		"Listening at {} (in container)...",
		consts::CONTAINER_ADDRESS,
	);
	actix_web::HttpServer::new(move || {
		let state = state.clone();
		let session_middleware =
			app::middleware::make_session_middleware(state.config());
		let identity_middleware = app::middleware::make_identity_middleware();

		actix_web::App::new()
			.app_data(state)
			.wrap(session_middleware)
			.wrap(identity_middleware)
			.wrap_fn(|r, service| {
				use actix_web::dev::Service as _;
				app::tera::register_resource_map(&r);
				service.call(r)
			})
			.service(app::service::index)
			.service(app::service::login_get)
			.service(app::service::login_post)
			.service(app::service::register_get)
			.service(app::service::register_post)
			.service(app::service::logout)
			.service(app::service::profile)
			.service(app::service::switch_f2f)
			.service(app::service::delete_account_get)
			.service(app::service::delete_account_post)
			.service(app::service::emails)
			.service(app::service::load_emails)
			.service(app::service::send_email_get)
			.service(app::service::send_email_post)
			.service(app::service::email)
			.service(app::service::friends)
			.service(app::service::add_friend_get)
			.service(app::service::add_friend_post)
			.service(app::service::delete_friend)
			.service(app::service::nodes_get)
			.service(app::service::nodes_post)
			.service(app::service::add_node_get)
			.service(app::service::add_node_post)
			.service(app::service::delete_node)
	})
	.bind(consts::CONTAINER_ADDRESS)
	.context("Failed to bind the application.")?
	.run()
	.await
	.context("Failed to run the application.")?;
	Ok(())
}
