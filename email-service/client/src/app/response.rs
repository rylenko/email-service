use super::error::{
	RedirectError, RedirectStaticError, RenderError, RenderFormErrorsError,
};

/// An auxiliary function for rendering templates and making responses with
/// ease.
pub(super) fn render(
	r: &actix_web::HttpRequest,
	template_name: &str,
	context: Option<tera::Context>,
	status: actix_web::http::StatusCode,
	csrf: bool,
) -> Result<actix_web::HttpResponse, RenderError> {
	// Make response builder
	let mut builder = actix_web::HttpResponse::build(status);
	builder.content_type("text/html");

	// Extending the base context with the `context`
	let mut full_context = super::tera::make_base_context(r)?;
	if let Some(c) = context {
		full_context.extend(c);
	}

	if csrf {
		// Generate a token, insert it into the template context and set a
		// cookie
		let token = super::csrf::generate_token()?;
		full_context.insert("csrf_token", &token);
		builder.cookie(super::csrf::make_cookie(r, &token));
	}

	// Get tera from state and render body
	let body = r
		.app_data::<actix_web::web::Data<crate::state::State>>()
		.unwrap()
		.tera()
		.render(template_name, &full_context)?;
	Ok(builder.body(body))
}

/// A shorthand for rendering form errors using `render`.
pub(super) fn render_form_errors(
	r: &actix_web::HttpRequest,
	template_name: &str,
	errors: &validator::ValidationErrors,
	context: Option<tera::Context>,
) -> Result<actix_web::HttpResponse, RenderFormErrorsError> {
	let mut full_context = context! {"form_errors" => &errors.field_errors()};
	if let Some(c) = context {
		full_context.extend(c);
	}
	Ok(render(
		r,
		template_name,
		Some(full_context),
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

/// Constructs a URL using `name` and `elements` and redirects to it.
pub(super) fn redirect<I, II>(
	r: &actix_web::HttpRequest,
	name: &str,
	elements: II,
) -> Result<actix_web::HttpResponse, RedirectError>
where
	I: AsRef<str>,
	II: IntoIterator<Item = I>,
{
	let url = r.url_for(name, elements)?;
	let header = (actix_web::http::header::LOCATION, url.as_str());
	Ok(actix_web::HttpResponse::MovedPermanently()
		.append_header(header)
		.finish())
}

/// Same as `redirect` but without elements.
pub(super) fn redirect_static(
	r: &actix_web::HttpRequest,
	name: &str,
) -> Result<actix_web::HttpResponse, RedirectStaticError> {
	const NO_ELEMENTS: [&str; 0] = [];
	let rv = redirect(r, name, NO_ELEMENTS)?;
	Ok(rv)
}
