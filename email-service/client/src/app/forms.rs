use validator::ValidationError;

type ValidationResult = std::result::Result<(), ValidationError>;

fn validate_csrf_token(
	s: &str,
	r: &actix_web::HttpRequest,
) -> ValidationResult {
	super::csrf::check_token(r, s).map_err(|_| ValidationError::new("invalid"))
}

#[derive(serde::Deserialize, validator::Validate)]
pub(crate) struct Login {
	#[validate(length(
		min = 3,
		max = 45,
		message = "Username length must be >= 3 and <= 45."
	))]
	pub username: String,
	#[validate(length(
		min = 6,
		max = 50,
		message = "Password length must be >= 6 and <= 50."
	))]
	pub password: String,
	#[validate(custom(
		function = "validate_csrf_token",
		arg = "&'v_a actix_web::HttpRequest",
		message = "CSRF token is invalid."
	))]
	csrf_token: String,
}

#[derive(serde::Deserialize, validator::Validate)]
pub(crate) struct Register {
	#[validate(
		custom(
			function = "Self::validate_username_unique",
			arg = "&'v_a crate::db::Db",
			message = "Username is not unique."
		),
		length(
			min = 3,
			max = 45,
			message = "Username length must be >= 3 and <= 45."
		)
	)]
	pub username: String,
	#[validate(length(
		min = 6,
		max = 50,
		message = "Password length must be >= 6 and <= 50."
	))]
	pub password: String,
	#[validate(must_match(
		other = "password",
		message = "Passwords must match."
	))]
	password_confirm: String,
	#[validate(custom(
		function = "Self::validate_private_key_pem_base64",
		message = "Private key is invalid.",
	))]
	private_key_pem_base64: Option<String>,
	#[validate(custom(
		function = "validate_csrf_token",
		arg = "&'v_a actix_web::HttpRequest",
		message = "CSRF token is invalid."
	))]
	csrf_token: String,
}

impl Register {
	#[must_use]
	pub fn get_private_key(
		&self,
	) -> Option<openssl::rsa::Rsa<openssl::pkey::Private>> {
		// Can use `Option::unwrap`, because private key validated in
		// `self.validate_private_key_pem_base64`.
		self.private_key_pem_base64.as_ref().map(|s| {
			super::keys::convert_pem_base64_to_private_key(s).unwrap()
		})
	}

	fn validate_username_unique(
		username: &str,
		db: &crate::db::Db,
	) -> ValidationResult {
		let f = db.check_user_exists(username);
		if futures::executor::block_on(f).unwrap() {
			return Err(ValidationError::new("not_unique"));
		}
		Ok(())
	}

	fn validate_private_key_pem_base64(s: &str) -> ValidationResult {
		super::keys::convert_pem_base64_to_private_key(s)
			.map_err(|_| ValidationError::new("invalid"))?;
		Ok(())
	}
}

#[derive(serde::Deserialize, validator::Validate)]
pub(crate) struct DeleteAccount {
	#[validate(
		custom(
			function = "Self::validate_password",
			arg = "&'v_a crate::raw_models::User",
			message = "Invalid password."
		),
		length(
			min = 6,
			max = 50,
			message = "Password length must be >= 6 and <= 50."
		)
	)]
	password: String,
	#[validate(custom(
		function = "validate_csrf_token",
		arg = "&'v_a actix_web::HttpRequest",
		message = "CSRF token is invalid."
	))]
	csrf_token: String,
}

impl DeleteAccount {
	fn validate_password(
		s: &str,
		user: &crate::raw_models::User,
	) -> ValidationResult {
		if s == user.password() {
			return Ok(());
		}
		Err(ValidationError::new("invalid"))
	}
}

#[derive(serde::Deserialize, validator::Validate)]
pub(crate) struct Friend {
	#[validate(
		custom(
			function = "Self::validate_username_unique",
			arg = "(&'v_a crate::db::Db, &'v_a crate::raw_models::User)",
			message = "Username is not unique.",
		),
		length(
			min = 3,
			max = 45,
			message = "Username length must be >= 3 and <= 45."
		)
	)]
	pub username: String,
	#[validate(
		custom(
			function = "Self::validate_public_key_pem_base64",
			message = "Public key is invalid."
		),
		custom(
			function = "Self::validate_public_key_unique",
			arg = "(&'v_a crate::db::Db, &'v_a crate::raw_models::User)",
			message = "Public key is not unique."
		)
	)]
	pub public_key_pem_base64: String,
	#[validate(custom(
		function = "validate_csrf_token",
		arg = "&'v_a actix_web::HttpRequest",
		message = "CSRF token is invalid."
	))]
	csrf_token: String,
}

impl Friend {
	fn validate_username_unique(
		s: &str,
		data: (&crate::db::Db, &crate::raw_models::User),
	) -> ValidationResult {
		let f = data.0.check_friend_exists_by_username(data.1, s);
		if futures::executor::block_on(f).unwrap() {
			return Err(ValidationError::new("not_unique"));
		}
		Ok(())
	}

	fn validate_public_key_pem_base64(s: &str) -> ValidationResult {
		super::keys::convert_pem_base64_to_public_key(s)
			.map_err(|_| ValidationError::new("invalid"))?;
		Ok(())
	}

	fn validate_public_key_unique(
		s: &str,
		data: (&crate::db::Db, &crate::raw_models::User),
	) -> ValidationResult {
		let f = data.0.check_friend_exists_by_public_key(data.1, s);
		if futures::executor::block_on(f).unwrap() {
			return Err(ValidationError::new("not_unique"));
		}
		Ok(())
	}
}

#[derive(serde::Deserialize, validator::Validate)]
pub(crate) struct Node {
	#[validate(
		custom(
			function = "Self::validate_address",
			message = "Address is invalid."
		),
		custom(
			function = "Self::validate_address_unique",
			message = "Address is not unique.",
			arg = "(&'v_a crate::db::Db, &'v_a crate::raw_models::User)",
		)
	)]
	pub address: String,
	pub password: String,
	#[validate(custom(
		function = "validate_csrf_token",
		arg = "&'v_a actix_web::HttpRequest",
		message = "CSRF token is invalid."
	))]
	csrf_token: String,
}

impl Node {
	fn validate_address(s: &str) -> ValidationResult {
		s.parse::<std::net::SocketAddr>()
			.map_err(|_| ValidationError::new("invalid"))?;
		Ok(())
	}

	fn validate_address_unique(
		s: &str,
		data: (&crate::db::Db, &crate::raw_models::User),
	) -> ValidationResult {
		let f = data.0.check_node_exists(data.1, s);
		if futures::executor::block_on(f).unwrap() {
			return Err(ValidationError::new("not_unique"));
		}
		Ok(())
	}
}

#[derive(serde::Deserialize, validator::Validate)]
pub(crate) struct Email {
	#[validate(custom(
		function = "Self::validate_recipient_public_key_pem_base64",
		message = "Recipient's public key is invalid"
	))]
	recipient_public_key_pem_base64: String,
	#[validate(length(
		min = 3,
		max = 200,
		message = "Title length must be >= 3 and <= 200."
	))]
	title: String,
	text: String,
	files: Option<Vec<super::multipart::File>>,
	#[validate(custom(
		function = "validate_csrf_token",
		arg = "&'v_a actix_web::HttpRequest",
		message = "CSRF token is invalid."
	))]
	csrf_token: String,
}

impl Email {
	common::accessor!(& title -> &str);

	#[must_use]
	pub(super) fn get_recipient_public_key(
		&self,
	) -> openssl::rsa::Rsa<openssl::pkey::Public> {
		// It must be validated in
		// [`validate_private_key`](Email::validate_private_key).
		// Therefore, we can use `Result::unwrap`.
		super::keys::convert_pem_base64_to_public_key(
			&self.recipient_public_key_pem_base64,
		)
		.unwrap()
	}

	#[must_use]
	pub(super) fn into_email_data(
		self,
		sender_username: String,
	) -> common::email::Data {
		common::email::Data::new(
			sender_username,
			self.title,
			self.text,
			self.files.map(|fs| fs.into_iter().map(Into::into).collect()),
		)
	}

	fn validate_recipient_public_key_pem_base64(s: &str) -> ValidationResult {
		super::keys::convert_pem_base64_to_public_key(s)
			.map_err(|_| ValidationError::new("invalid"))?;
		Ok(())
	}
}

#[derive(serde::Deserialize, validator::Validate)]
pub(crate) struct Csrf {
	#[validate(custom(
		function = "validate_csrf_token",
		arg = "&'v_a actix_web::HttpRequest",
		message = "CSRF token is invalid."
	))]
	csrf_token: String,
}
