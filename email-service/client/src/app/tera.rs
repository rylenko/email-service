use super::error::MakeTeraBaseContextError;

thread_local! {
	// Used to build URLs in templates without `actix_web::HttpRequest`.
	static RESOURCE_MAP:
		std::cell::RefCell<Option<actix_web::dev::ResourceMap>>
		= std::cell::RefCell::new(None);
}

/// Returns the already bound to the template folder `tera::Tera`.
#[must_use]
pub(crate) fn make_tera() -> tera::Tera {
	let mut tera = tera::Tera::new(crate::consts::TERA_DIR_STR).unwrap();
	tera.register_function("url_for", url_for);
	tera
}

/// Extracts a resource map [`r`](actix_web::dev::ServiceRequest) for
/// `url_for` usage in templates.
pub(crate) fn register_resource_map(r: &actix_web::dev::ServiceRequest) {
	RESOURCE_MAP.with(|m| {
		m.borrow_mut().get_or_insert_with(|| r.resource_map().clone());
	});
}

/// Creates a base context for the base template "base.html", which cannot be
/// retrieved without [`actix_web::HttpRequest`].
pub(super) fn make_base_context(
	r: &actix_web::HttpRequest,
) -> Result<tera::Context, MakeTeraBaseContextError> {
	let dark_theme = r
		.app_data::<actix_web::web::Data<crate::state::State>>()
		.unwrap()
		.config()
		.dark_theme();
	let user = super::auth::get_current_user(r)?;
	let flashes = super::flash::pop_all(r)?;
	Ok(context! {
		"dark_theme" => &dark_theme,
		"user" => &user,
		"path" => r.path(),
		"flashes" => &flashes,
	})
}

/// Used to build URLs in templates.
///
/// Before using it, it is important to make sure that the resource map has
/// been registered using `register_resource_map`.
fn url_for(
	args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
	let name = match args.get("name") {
		Some(v) => match v.as_str() {
			Some(s) => s,
			None => return Err(tera::Error::msg("`name` shoud be a string.")),
		},
		None => return Err(tera::Error::msg("`name` argument not found.")),
	};
	let elements = match args.get("elements") {
		Some(value) => match value.as_array() {
			Some(vec) => {
				let mut elements = Vec::with_capacity(vec.len());
				for value in vec {
					let Some(s) = value.as_str() else {
						return Err(tera::Error::msg(
							"`elements` should contain only strings.",
						));
					};
					elements.push(s);
				}
				elements
			}
			None => {
				return Err(tera::Error::msg("`elements` should be an array."))
			}
		},
		None => vec![],
	};

	RESOURCE_MAP.with(|m| {
		let test_r = actix_web::test::TestRequest::default().to_http_request();
		let url = m
			.borrow()
			.as_ref()
			.ok_or_else(|| {
				tera::Error::msg("`url_for` should be called in r context.")
			})?
			.url_for(&test_r, name, elements)
			.map_err(|_| tera::Error::msg("Resource not found."))?;

		Ok(tera::Value::String(url.path().to_owned()))
	})
}
