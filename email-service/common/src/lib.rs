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

#[macro_use]
mod macros;
pub mod consts;
pub mod crypto;
pub mod email;
#[allow(clippy::module_name_repetitions)]
pub mod error;
pub mod helpers;
pub mod package;
