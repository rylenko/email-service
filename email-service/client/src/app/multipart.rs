use super::error::{
	ExtractFieldValueError, ExtractFileFieldValueError, ExtractMultipartError,
};

/// A type that can be in the form along with other fields.
#[derive(serde::Deserialize)]
pub(super) struct File {
	name: String,
	data: Vec<u8>,
}

impl From<File> for common::email::File {
	#[inline]
	#[must_use]
	fn from(f: File) -> Self {
		Self::new(f.name, f.data)
	}
}

/// Extracts fields with their values from [`actix_multipart::Multipart`] to
/// [`serde_json::Value`]s and deserializes them in `T`.
pub(super) async fn extract<T: serde::de::DeserializeOwned>(
	mut multipart: actix_multipart::Multipart,
) -> Result<T, ExtractMultipartError> {
	use futures::TryStreamExt as _;

	// This contains the usual fields
	let mut fields = serde_json::Map::new();
	// This contains fields with files (possibly more than one).
	let mut file_fields =
		std::collections::HashMap::<String, Vec<serde_json::Value>>::new();

	// Extract the usual fields and the fields with files
	while let Ok(Some(field)) = multipart.try_next().await {
		if let Some(field_name) =
			field.content_disposition().get_name().map(ToOwned::to_owned)
		{
			if let Some(file_name) = field
				.content_disposition()
				.get_filename()
				.map(ToOwned::to_owned)
			{
				// Extract file map and insert it into `file_fields`
				if let Some(value) = extract_file_field_value(field).await? {
					let mut map = serde_json::Map::new();
					map.insert(
						"name".to_owned(),
						serde_json::Value::String(file_name),
					);
					map.insert("data".to_owned(), value);
					file_fields
						.entry(field_name)
						.or_default()
						.push(serde_json::Value::Object(map));
				}
			} else if let Some(value) = extract_field_value(field).await? {
				fields.insert(field_name, value);
			}
		}
	}

	// Moving file fields from `file_maps` to `fields_map`
	for (field_name, files) in file_fields {
		fields.insert(field_name, serde_json::Value::Array(files));
	}
	// Attempting to convert extracted values to `T'
	let rv = serde_json::from_value(serde_json::Value::Object(fields))?;
	Ok(rv)
}

/// Used to extract file bytes from a `field`.
///
/// The return value is `serde_json::Value::Array`, which contains
/// `serde_json::Value::Number`, which represents a byte. Also, the Return
/// Value can be [`None`] if the form field is empty.
async fn extract_file_field_value(
	mut field: actix_multipart::Field,
) -> Result<Option<serde_json::Value>, ExtractFileFieldValueError> {
	use futures::StreamExt as _;

	let mut data = Vec::new();
	while let Some(chunk) = field.next().await {
		for byte in chunk? {
			data.push(serde_json::Value::Number(serde_json::Number::from(
				byte,
			)));
		}
	}
	if data.is_empty() {
		return Ok::<_, ExtractFileFieldValueError>(None);
	}
	Ok(Some(serde_json::Value::Array(data)))
}

/// Used to retrieve value from the `field`.
async fn extract_field_value(
	mut field: actix_multipart::Field,
) -> Result<Option<serde_json::Value>, ExtractFieldValueError> {
	use futures::StreamExt as _;
	match field.next().await {
		Some(chunk_result) => {
			let chunk = chunk_result?;
			let s = std::str::from_utf8(&chunk)?;
			Ok::<_, ExtractFieldValueError>(Some(convert_str_to_value(s)))
		}
		None => Ok(None),
	}
}

#[must_use]
fn convert_str_to_value(s: &str) -> serde_json::Value {
	match s.parse::<i64>() {
		Ok(n) => serde_json::Value::Number(serde_json::Number::from(n)),
		Err(_) => match s {
			"true" => serde_json::Value::Bool(true),
			"false" => serde_json::Value::Bool(false),
			_ => serde_json::Value::String(s.to_owned()),
		},
	}
}
