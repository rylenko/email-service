use crate::{
	// Use `crate` as `common` to call macros
	self as common,
	error::{
		CreateDbPoolError, DeserializeJsonFromFileError, SendEmailToNodesError,
	},
};

pub type DbPool = diesel_async::pooled_connection::deadpool::Pool<
	diesel_async::AsyncPgConnection,
>;

/// Creates a database connection pool at `url`.
pub fn create_db_pool() -> Result<DbPool, CreateDbPoolError> {
	let config = diesel_async::pooled_connection::AsyncDieselConnectionManager::<
		diesel_async::AsyncPgConnection,
	>::new(&*crate::consts::DB_URL);
	let pool =
		diesel_async::pooled_connection::deadpool::Pool::builder(config)
			.build()?;
	Ok(pool)
}

/// Reads the file and uses [`serde_json`] to deserialize the content.
pub async fn deserialize_json_from_file<T>(
	path: &std::path::Path,
) -> Result<T, DeserializeJsonFromFileError>
where
	T: serde::de::DeserializeOwned,
{
	let content = tokio::fs::read(path).await?;
	let owned = serde_json::from_slice(&content)?;
	Ok(owned)
}

/// In multi-threaded mode, sends each node a `package` which contains
/// `email::Email` bytes. After sending, it waits for a response from the node.
///
/// # Debug panic
///
/// If `package.action()` is not `package::Action::SendEmail`.
pub async fn send_email_to_nodes<N, IN>(
	package: crate::package::Package,
	nodes: IN,
	nodes_len: usize,
	proxy: Option<std::net::SocketAddr>,
) -> Result<usize, SendEmailToNodesError>
where
	N: Into<(std::net::SocketAddr, Option<String>)>,
	IN: IntoIterator<Item = N>,
{
	debug_assert_eq!(package.action(), crate::package::Action::SendEmail);

	let mut count = 0;
	let package = std::sync::Arc::new(tokio::sync::Mutex::new(package));
	let mut futures = Vec::with_capacity(nodes_len);

	for node in nodes {
		let (address, password) = node.into();
		let package = package.clone();

		let future = tokio::spawn(async move {
			let mut stream =
				crate::connect_or_else!(address, proxy, return false);

			// Send email package
			let mut lock = package.lock().await;
			lock.set_password(password.as_deref());
			crate::send_package_or_else!(
				lock,
				&mut stream,
				address,
				return false
			);
			drop(lock);

			// Receive a response
			let response = crate::receive_package_or_else!(
				&mut stream,
				address,
				None,
				Some(crate::set![
					crate::package::Action::SendEmailSuccess,
					crate::package::Action::SendEmailFail,
				]),
				return false,
			);
			if response.action() == common::package::Action::SendEmailSuccess {
				crate::debug!("New email successfully added in {}.", address);
				true
			} else {
				crate::debug!("New email has not been added in {}.", address);
				false
			}
		});
		futures.push(future);
	}

	// Join tasks
	for f in futures::future::join_all(futures).await {
		if f? {
			count += 1;
		}
	}
	Ok(count)
}
