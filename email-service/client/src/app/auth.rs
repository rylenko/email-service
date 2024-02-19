use super::error::{
	GetCurrentUserError, LoginUserError, ValidateLoggedInError,
	ValidateLoggedOutError,
};

pub(super) fn get_current_user(
	r: &actix_web::HttpRequest,
) -> Result<Option<crate::raw_models::User>, GetCurrentUserError> {
	use actix_identity::IdentityExt as _;

	if let Ok(id) = r.get_identity() {
		let data = id.id()?;
		Ok(serde_json::from_str(&data)?)
	} else {
		Ok(None)
	}
}

pub(super) fn login_user(
	r: &actix_web::HttpRequest,
	u: &crate::raw_models::User,
) -> Result<(), LoginUserError> {
	use actix_web::HttpMessage as _;

	let data = serde_json::to_string(u)?;
	actix_identity::Identity::login(&r.extensions(), data)?;
	Ok(())
}

/// Validates that the user is logged in.
pub(super) fn validate_logged_in(
	r: &actix_web::HttpRequest,
) -> Result<(), ValidateLoggedInError> {
	use actix_identity::IdentityExt as _;
	r.get_identity().map_err(|_| ValidateLoggedInError::LoggedOut)?;
	Ok(())
}

/// Validates that the user is logged out.
pub(super) fn validate_logged_out(
	r: &actix_web::HttpRequest,
) -> Result<(), ValidateLoggedOutError> {
	use actix_identity::IdentityExt as _;
	if r.get_identity().is_ok() {
		return Err(ValidateLoggedOutError::LoggedIn);
	}
	Ok(())
}
