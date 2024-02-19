use super::error::{CheckCsrfTokenError, GenerateCsrfTokenError};

/// Compares `token` and the value specified in the cookie.
pub(super) fn check_token(
	r: &actix_web::HttpRequest,
	token: &str,
) -> Result<(), CheckCsrfTokenError> {
	let cookies = r.cookies()?;
	cookies
		.iter()
		.find(|c| c.name() == crate::consts::CSRF_COOKIE_NAME)
		.ok_or(CheckCsrfTokenError::NotFound)
		.map(actix_web::cookie::Cookie::value)
		.and_then(|value| {
			if value == token {
				return Ok(());
			}
			Err(CheckCsrfTokenError::Invalid)
		})
}

pub(super) fn generate_token() -> Result<String, GenerateCsrfTokenError> {
	let bytes = common::crypto::generate_random_bytes(None)?;
	Ok(base64::encode(bytes))
}

#[must_use = "Add cookie to response."]
pub(super) fn make_cookie<'a>(
	r: &actix_web::HttpRequest,
	token: &'a str,
) -> actix_web::cookie::Cookie<'a> {
	let domain =
		r.connection_info().host().split(':').next().unwrap().to_owned();
	actix_web::cookie::Cookie::build(crate::consts::CSRF_COOKIE_NAME, token)
		.domain(domain)
		.path("/")
		.secure(true)
		.http_only(true)
		.same_site(actix_web::cookie::SameSite::Strict)
		.finish()
}
