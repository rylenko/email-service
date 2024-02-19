use anyhow::{Context as _, Result};

/// Entry point for `stream` data processing.
pub(crate) async fn stream(
	mut stream: tokio::net::TcpStream,
	from_address: std::net::SocketAddr,
	state: &crate::state::State,
) -> Result<()> {
	use common::package::Action;
	let package = common::receive_package_or_else!(
		&mut stream,
		from_address,
		state.config().password(),
		None,
		return Ok(()),
	);
	match package.action() {
		Action::CheckConnection => check_connection(stream)
			.await
			.context("Failed to handle connection check."),
		Action::GetEmail => get_email(stream, state, package)
			.await
			.context("Failed to handle email getting."),
		Action::GetEmailsCount => get_emails_count(stream, state, package)
			.await
			.context("Failed to handle emails count getting."),
		Action::SendEmail => send_email(stream, state, package)
			.await
			.context("Failed to handle email sending."),
		_ => Ok(()),
	}
}

async fn check_connection(mut stream: tokio::net::TcpStream) -> Result<()> {
	common::package::Package::new(
		None,
		common::package::Action::CheckConnectionSuccess,
		vec![],
	)
	.send(&mut stream)
	.await
	.context("Failed to send a package.")
}

async fn get_email(
	mut stream: tokio::net::TcpStream,
	state: &crate::state::State,
	package: common::package::Package,
) -> Result<()> {
	let fail_response = common::package::Package::new(
		None,
		common::package::Action::GetEmailFail,
		vec![],
	);

	// Deserialize package data
	let (index, recipient_public_key_hash): (i64, [u8; 32]) =
		match bincode::deserialize(package.data()) {
			Ok((n, h)) => (n, h),
			Err(_) => {
				return fail_response
					.send(&mut stream)
					.await
					.context("Failed to send fail response.");
			}
		};

	// Get encrypted email bytes and send response
	let response = match state
		.db()
		.get_email_bytes(index, &recipient_public_key_hash)
		.await
	{
		Ok(b) => common::package::Package::new(
			None,
			common::package::Action::GetEmailSuccess,
			b,
		),
		_ => fail_response,
	};
	response.send(&mut stream).await.context("Failed to send response.")
}

async fn get_emails_count(
	mut stream: tokio::net::TcpStream,
	state: &crate::state::State,
	package: common::package::Package,
) -> Result<()> {
	use std::convert::TryInto as _;

	let fail_response = common::package::Package::new(
		None,
		common::package::Action::GetEmailsCountFail,
		vec![],
	);

	// Convert package data to hash
	let Ok(hash) = package.data().try_into() else {
		return fail_response
			.send(&mut stream)
			.await
			.context("Failed to send a fail response.");
	};

	// Get emails count and send response
	let response = match state.db().get_emails_count(hash).await {
		Ok(ref c) => common::package::Package::new(
			None,
			common::package::Action::GetEmailsCountSuccess,
			bincode::serialize(c).context("Failed to serialize.")?,
		),
		Err(_) => fail_response,
	};
	response.send(&mut stream).await.context("Failed to send a package.")
}

/// Attempts to add a email to the database. If successful, calls
/// `common::send_email_to_nodes`.
async fn send_email(
	mut stream: tokio::net::TcpStream,
	state: &crate::state::State,
	package: common::package::Package,
) -> Result<()> {
	let email: common::email::Email =
		if let Ok(ee) = bincode::deserialize(package.data()) {
			ee
		} else {
			return Err(anyhow::anyhow!("Invalid data."));
		};
	if !email.check_encrypted_integrity() {
		return Err(anyhow::anyhow!("Invalid email."));
	}
	let response = match state.db().add_email(&email).await {
		Ok(()) => common::package::Package::new(
			None,
			common::package::Action::SendEmailSuccess,
			vec![],
		),
		Err(_) => common::package::Package::new(
			None,
			common::package::Action::SendEmailFail,
			vec![],
		),
	};
	response.send(&mut stream).await.context("Failed to send a response")?;
	if response.action() == common::package::Action::SendEmailSuccess {
		common::debug!("The email was successfully added.");

		if let Some(on) = state.config().other_nodes() {
			match common::helpers::send_email_to_nodes(
				package,
				on.clone(),
				on.len(),
				None,
			)
			.await
			.context("Failed to send email to nodes.")?
			{
				0 => common::debug!(
					"The email was not forwarded to other nodes."
				),
				c => common::debug!(
					"The email was successfully forwarded to {c} other nodes.",
				),
			}
		}
	}
	Ok(())
}
