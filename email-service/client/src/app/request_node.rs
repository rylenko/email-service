use super::error::{GetNodeEmailsCountError, LoadNodeEmailsError};

/// Used in `app::service::load_emails` in multi-threaded mode to load
/// emails from each node. Returns the number of loaded emails.
pub(super) async fn load_emails(
	node: crate::raw_models::Node,
	s: actix_web::web::Data<crate::state::State>,
	user: std::sync::Arc<crate::raw_models::User>,
	private_key: std::sync::Arc<openssl::rsa::Rsa<openssl::pkey::Private>>,
) -> Result<u8, LoadNodeEmailsError> {
	// Get emails count
	let node = std::sync::Arc::new(node);
	let mut stream = common::connect_or_else!(
		node.address(),
		s.config().proxy(),
		return Ok(0),
	);
	let count = get_emails_count(&node, &private_key, &mut stream).await?;
	if count == 0 {
		return Ok(0);
	}

	// Request to add each email until we reach the limit
	let public_key_hash =
		common::crypto::hash(private_key.public_key_to_pem()?);
	let mut added_count = 0u8;
	for index in 0..count {
		let mut stream = common::connect_or_else!(
			node.address(),
			s.config().proxy(),
			return Ok(0),
		);

		// Send a request
		let package = common::package::Package::new(
			node.password(),
			common::package::Action::GetEmail,
			bincode::serialize(&(index, &public_key_hash))?,
		);
		common::send_package_or_else!(
			package,
			&mut stream,
			node.address(),
			continue,
		);

		// Receive and validate a response
		let response = common::receive_package_or_else!(
			&mut stream,
			node.address(),
			None,
			Some(common::set![
				common::package::Action::GetEmailSuccess,
				common::package::Action::GetEmailFail,
			]),
			return Ok(0),
		);
		if response.action() == common::package::Action::GetEmailFail {
			common::debug!("Failed to get email from {}.", node.address());
			continue;
		}

		// Deserialize, decrypt and validate an email
		let Ok(mut email) =
			bincode::deserialize::<common::email::Email>(response.data())
		else {
			continue;
		};
		if email.decrypt(&private_key).is_err()
			|| !matches!(email.check_decrypted_integrity(), Ok(true))
			|| s.db()
				.check_email_exists(&user, &email)
				.await
				.map_err(LoadNodeEmailsError::CheckEmailExists)?
		{
			continue;
		}

		// Check F2F
		//
		// We can use `Option::unwrap` because integrity check.
		let sender_public_key_pem = email.sender_public_key_pem().unwrap();
		let f2f = s
			.db()
			.check_user_f2f(&user)
			.await
			.map_err(LoadNodeEmailsError::CheckUserF2f)?;
		let sender_public_key_pem_base64 =
			&base64::encode(sender_public_key_pem);
		let friend_exists_by_public_key = s
			.db()
			.check_friend_exists_by_public_key(
				&user,
				sender_public_key_pem_base64,
			)
			.await
			.map_err(LoadNodeEmailsError::CheckFriendExistsByPublicKey)?;
		if f2f && !friend_exists_by_public_key {
			continue;
		}

		// Add a new email
		if s.db().add_email(&user, &email).await.is_err() {
			common::debug!(
				"Failed to load an email from {} to the database.",
				node.address()
			);
			continue;
		};
		added_count += 1;
		if added_count == crate::consts::NEW_EMAILS_FROM_NODE_LIMIT {
			break;
		}
	}

	common::debug!(
		"{} new emails loaded from {}.",
		added_count,
		node.address()
	);
	Ok(added_count)
}

/// Used in `app::service::nodes_post` to check connection with each
/// node. Returns the address, the status of the check, and the reason why the
/// check failed.
pub(super) async fn check_connection(
	s: actix_web::web::Data<crate::state::State>,
	node: crate::raw_models::Node,
) -> (std::net::SocketAddr, Option<&'static str>) {
	let mut stream = common::connect_or_else!(
		node.address(),
		s.config().proxy(),
		return (node.address(), Some("Failed to connect.")),
	);
	// Make and send the package
	let package = common::package::Package::new(
		node.password(),
		common::package::Action::CheckConnection,
		vec![],
	);
	common::send_package_or_else!(
		package,
		&mut stream,
		node.address(),
		return (node.address(), Some("Failed to connect.")),
	);
	// Receive a response
	let response = common::receive_package_or_else!(
		&mut stream,
		node.address(),
		None,
		Some(common::set![
			common::package::Action::InvalidPassword,
			common::package::Action::CheckConnectionSuccess,
		]),
		return (node.address(), Some("Failed to connect.")),
	);
	// Return status
	match response.action() {
		common::package::Action::InvalidPassword => {
			(node.address(), Some("Invalid password."))
		}
		_ => (node.address(), None),
	}
}

async fn get_emails_count(
	node: &crate::raw_models::Node,
	private_key: &openssl::rsa::Rsa<openssl::pkey::Private>,
	stream: &mut tokio::net::TcpStream,
) -> Result<i64, GetNodeEmailsCountError> {
	let public_key_hash =
		common::crypto::hash(private_key.public_key_to_pem()?);
	let package = common::package::Package::new(
		node.password(),
		common::package::Action::GetEmailsCount,
		public_key_hash,
	);
	common::send_package_or_else!(
		package,
		stream,
		node.address(),
		return Ok::<_, GetNodeEmailsCountError>(0),
	);
	let response = common::receive_package_or_else!(
		stream,
		node.address(),
		None,
		Some(common::set![
			common::package::Action::GetEmailsCountSuccess,
			common::package::Action::GetEmailsCountFail,
		]),
		return Ok(0),
	);
	if response.action() == common::package::Action::GetEmailsCountFail {
		common::debug!(
			"Failed to get the count of emails from {}.",
			node.address()
		);
		return Ok(0);
	}
	if let Ok(c) = bincode::deserialize(response.data()) {
		Ok(c)
	} else {
		common::debug!(
			"Received invalid package data from {}.",
			node.address()
		);
		Ok(0)
	}
}
