/// Returns [`actix_session::CookieSession`] with the secret key from the
/// config
#[must_use]
pub(crate) fn make_session_middleware(
	config: &crate::config::Config,
) -> actix_session::SessionMiddleware<actix_session::storage::CookieSessionStore>
{
	let secret_key_bytes = config.secret_key().as_bytes();
	actix_session::SessionMiddleware::new(
		actix_session::storage::CookieSessionStore::default(),
		actix_web::cookie::Key::from(secret_key_bytes),
	)
}

/// Returns [`actix_identity::IdentityMiddleware`]
#[inline]
#[must_use]
pub(crate) fn make_identity_middleware() -> actix_identity::IdentityMiddleware
{
	actix_identity::IdentityMiddleware::default()
}
