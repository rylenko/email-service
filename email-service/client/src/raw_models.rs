/// This structure is different from other raw model structures and contains
/// minimal information because it is deserialized in cookies. If you need any
/// additional information from the database, use `client::Client`.
#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct User {
	id: i32,
	username: String,
	password: String,
}

impl User {
	common::accessor!(copy id -> i32);

	common::accessor!(& username -> &str);

	common::accessor!(& password -> &str);

	#[inline]
	#[must_use]
	pub fn new(id: i32, username: String, password: String) -> Self {
		Self { id, username, password }
	}

	/// Returns the [cipher](AesCipher) we use to encrypt most data.
	#[must_use]
	pub fn make_aes_cipher(&self) -> common::crypto::AesCipher {
		let key =
			common::crypto::hash_with_salt(&self.password, &self.username);
		common::crypto::AesCipher::new(key.to_vec())
	}
}

/// Same as `models::Email`, but with raw decrypted data.
#[derive(serde::Serialize)]
pub(crate) struct Email {
	id: i32,
	sender_public_key: String,
	data: common::email::Data,
}

impl Email {
	common::accessor!(& sender_public_key -> &str);

	common::accessor!(& data -> &common::email::Data);

	#[inline]
	#[must_use]
	pub fn new(
		id: i32,
		sender_public_key: String,
		data: common::email::Data,
	) -> Self {
		Self { id, sender_public_key, data }
	}
}

/// Same as `models::Friend`, but with raw decrypted data.
#[derive(serde::Serialize)]
pub(crate) struct Friend {
	id: i32,
	username: String,
	public_key: String,
}

impl Friend {
	common::accessor!(& username -> &str);

	#[inline]
	#[must_use]
	pub fn new(id: i32, username: String, public_key: String) -> Self {
		Self { id, username, public_key }
	}
}

/// Same as `models::Node`, but with raw decryped data.
#[derive(Clone, serde::Serialize)]
pub(crate) struct Node {
	id: i32,
	address: std::net::SocketAddr,
	password: Option<String>,
}

impl Node {
	common::accessor!(copy id -> i32);

	common::accessor!(copy address -> std::net::SocketAddr);

	common::accessor!(as_deref password -> Option<&str>);

	#[inline]
	#[must_use]
	pub fn new(
		id: i32,
		address: std::net::SocketAddr,
		password: Option<String>,
	) -> Self {
		Self { id, address, password }
	}
}

impl From<Node> for (std::net::SocketAddr, Option<String>) {
	#[inline]
	fn from(n: Node) -> (std::net::SocketAddr, Option<String>) {
		(n.address, n.password)
	}
}
