use super::error::{
	ConvertPemBase64ToPrivateKeyError, ConvertPemBase64ToPublicKeyError,
};

/// Converts the Base-64 encoded PEM into a
/// [`private key`](openssl::rsa::Rsa<openssl::pkey::Private>).
pub(super) fn convert_pem_base64_to_private_key(
	s: &str,
) -> Result<
	openssl::rsa::Rsa<openssl::pkey::Private>,
	ConvertPemBase64ToPrivateKeyError,
> {
	let pem = base64::decode(s)?;
	let key = openssl::rsa::Rsa::private_key_from_pem(&pem)?;
	Ok(key)
}

/// Converts the Base-64 encoded PEM into a
/// [`public key`](openssl::rsa::Rsa<openssl::pkey::Public>).
pub(super) fn convert_pem_base64_to_public_key(
	s: &str,
) -> Result<
	openssl::rsa::Rsa<openssl::pkey::Public>,
	ConvertPemBase64ToPublicKeyError,
> {
	let pem = base64::decode(s)?;
	let key = openssl::rsa::Rsa::public_key_from_pem(&pem)?;
	Ok(key)
}
