#[macro_use]
mod macros;
mod auth;
mod csrf;
#[allow(clippy::module_name_repetitions)]
mod error;
mod flash;
mod forms;
mod keys;
pub(crate) mod middleware;
mod multipart;
pub(crate) mod pagination;
mod qrcode;
mod request_node;
mod response;
#[allow(clippy::unused_async)]
pub(crate) mod service;
pub(crate) mod tera;
