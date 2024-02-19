/// Simplification to create [`tera::Context`].
macro_rules! context {
	($($key:expr => $value:expr),* $(,)?) => {{
		let mut context = tera::Context::new();
		$( context.insert($key, $value); )*
		context
	}};
}

/// Simplification to create [`validator::ValidationErrors`].
macro_rules! validation_errors {
	($($name:expr => $message:expr),* $(,)?) => {{
		let mut errors = validator::ValidationErrors::new();
		$(
			let mut error = validator::ValidationError::new($name);
			error.message = Some($message.into());
			errors.add("__all__", error);
		)*
		errors
	}};
}
