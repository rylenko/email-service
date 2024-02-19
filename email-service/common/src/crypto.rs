use crate::error::{
	AesDecryptBase64Error, AesDecryptError, AesDecryptStringError,
	AesEncryptError, GenerateRandomBytesError,
};

pub struct AesCipher<'a> {
	inner: openssl::symm::Cipher,
	key: std::borrow::Cow<'a, [u8]>,
}

impl<'a> AesCipher<'a> {
	#[must_use]
	pub fn new<K: Into<std::borrow::Cow<'a, [u8]>>>(key: K) -> Self {
		Self { inner: openssl::symm::Cipher::aes_256_gcm(), key: key.into() }
	}

	pub fn encrypt<D>(&self, data: D) -> Result<Vec<u8>, AesEncryptError>
	where
		D: AsRef<[u8]>,
	{
		let iv = generate_random_bytes(Some(16))?;
		let mut tag = [0u8; 16];
		let rv = openssl::symm::encrypt_aead(
			self.inner,
			&self.key,
			Some(&iv),
			&[],
			data.as_ref(),
			&mut tag,
		)?;
		Ok([iv, rv, tag.into()].concat())
	}

	pub fn decrypt(&self, slice: &[u8]) -> Result<Vec<u8>, AesDecryptError> {
		let iv = &slice[..16];
		let data = &slice[16..slice.len() - 16];
		let tag = &slice[slice.len() - 16..];
		let rv = openssl::symm::decrypt_aead(
			self.inner,
			&self.key,
			Some(iv),
			&[],
			data,
			tag,
		)?;
		Ok(rv)
	}

	pub fn decrypt_string(
		&self,
		data: &[u8],
	) -> Result<String, AesDecryptStringError> {
		let decrypted_data = self.decrypt(data)?;
		let s = String::from_utf8(decrypted_data)?;
		Ok(s)
	}

	pub fn decrypt_base64(
		&self,
		data: &[u8],
	) -> Result<String, AesDecryptBase64Error> {
		let decrypted_data = self.decrypt(data)?;
		Ok(base64::encode(decrypted_data))
	}
}

/// Generates a random bytes with `getrandom::getrandom`.
///
/// # Params
///
/// Default `length` is `consts::DEFAULT_RANDOM_BYTES_LENGTH`.
pub fn generate_random_bytes(
	length: Option<usize>,
) -> Result<Vec<u8>, GenerateRandomBytesError> {
	let length_raw =
		length.unwrap_or(crate::consts::DEFAULT_RANDOM_BYTES_LENGTH);
	let mut random_bytes = vec![0u8; length_raw];
	getrandom::getrandom(&mut random_bytes)?;
	Ok(random_bytes)
}

/// SHA-256 hashing.
#[inline]
#[must_use]
pub fn hash<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
	openssl::sha::sha256(data.as_ref())
}

/// SHA-256 hashing with salt.
#[must_use]
pub fn hash_with_salt<T, U>(data: T, salt: U) -> [u8; 32]
where
	T: AsRef<[u8]>,
	U: AsRef<[u8]>,
{
	hash([data.as_ref(), salt.as_ref()].concat())
}
