#![deny(clippy::correctness)]
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

mod config;
mod consts;
mod db;
mod handle;
mod models;
mod schema;
mod state;
mod task;

use anyhow::{Context as _, Result};

pub async fn launch() -> Result<()> {
	// Make a default state
	common::debug!("Making a default state...");
	let state: &'static state::State = Box::leak(Box::new(
		state::State::try_default()
			.await
			.context("Failed to make a default state.")?,
	));

	// Spawn tasks
	tokio::spawn(task::delete_old_emails_task(state));

	// Create and bind a listener
	let listener =
		tokio::net::TcpListener::bind(crate::consts::CONTAINER_ADDRESS)
			.await
			.context("Failed to bind listener.")?;
	println!(
		"Listening at {} (in container)...",
		crate::consts::CONTAINER_ADDRESS,
	);

	// Accept connections
	loop {
		let (stream, from_address) = listener.accept().await?;
		common::debug!("New connection from {from_address}.");
		tokio::spawn(async move {
			if let Err(e) = handle::stream(stream, from_address, state).await {
				common::debug!(
					"Failed to handle {}:\n{:?}\n",
					from_address,
					e
				);
			}
		});
	}
}
