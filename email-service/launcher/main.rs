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

use anyhow::{Context as _, Result};

/// Parses the executable component name from [`std::env::args`], which is
/// either `node` or `client`.
#[must_use]
fn extract_executable_name_from_args() -> String {
	let name = std::env::args().nth(1).unwrap_or_else(|| {
		eprintln!("Enter the name.");
		std::process::exit(1);
	});
	if name != "node" && name != "client" {
		eprintln!("Invalid executable name.");
		std::process::exit(1);
	}
	name
}

#[tokio::main]
async fn main() -> Result<()> {
	if extract_executable_name_from_args() == "client" {
		client::launch().await.context("Failed to run client.")
	} else {
		node::launch().await.context("Failed to run node.")
	}
}
