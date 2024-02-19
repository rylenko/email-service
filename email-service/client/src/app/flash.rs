use super::error::{
	AddFlashError, AddFormErrorFlashesError, PopAllFlashesError,
};

/// Adds a message to the [`r`](actix_web::HttpRequest) session with
/// a category, which you can then get with `pop_all` and show
/// in the template.
pub(super) fn add(
	r: &actix_web::HttpRequest,
	message: &str,
	category: &str,
) -> Result<(), AddFlashError> {
	use actix_session::SessionExt as _;

	let session = r.get_session();
	let mut flashes: Vec<_> = session.get("_flashes")?.unwrap_or_default();
	flashes.push((message.to_owned(), category.to_owned()));
	session.insert("_flashes", flashes)?;
	Ok(())
}

/// A shorthand for flashing form errors using `add`.
pub(super) fn add_form_errors(
	r: &actix_web::HttpRequest,
	errors: &validator::ValidationErrors,
) -> Result<(), AddFormErrorFlashesError> {
	for field_errors in errors.field_errors().values() {
		for error in *field_errors {
			add(r, error.message.as_ref().unwrap(), "danger")?;
		}
	}
	Ok(())
}

/// Used to retrieve messages with categories made with `add` from the session.
pub(super) fn pop_all(
	r: &actix_web::HttpRequest,
) -> Result<Option<Vec<(String, String)>>, PopAllFlashesError> {
	use actix_session::SessionExt as _;

	let session = r.get_session();
	let flashes = session.get("_flashes")?;
	if flashes.is_some() {
		session.remove("_flashes");
	}
	Ok(flashes)
}
