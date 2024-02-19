use crate::error::{
	PackageIsTooBigError, ReceivePackageBytesError, ReceivePackageError,
	SendPackageError,
};

/// `Package` action.
#[derive(
	Clone,
	Copy,
	Debug,
	Eq,
	Hash,
	PartialEq,
	serde::Deserialize,
	serde::Serialize,
)]
pub enum Action {
	CheckConnection,
	CheckConnectionSuccess,
	GetEmail,
	GetEmailSuccess,
	GetEmailFail,
	GetEmailsCount,
	GetEmailsCountSuccess,
	GetEmailsCountFail,
	InvalidPassword,
	SendEmail,
	SendEmailSuccess,
	SendEmailFail,
}

/// A package for exchanging `self.data` using the `self.send` and
/// `Self::receive` methods.
///
/// # Examples
///
/// Send:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// # let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8888").await?;
/// let package = common::package::Package::new(
///     None,
///     common::package::Action::SendEmail,
///     vec![0, 1, 2],
/// );
/// package.send(&mut stream).await?;
/// # Ok(())
/// # }
/// ```
///
/// Receive:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// # let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8888").await?;
/// let mut package = common::package::Package::receive(
///     &mut stream,
///     None,
///     Some(common::set![common::package::Action::SendEmail]),
/// ).await?;
/// println!("Data: {:?}", package.data());
/// # Ok(())
/// # }
/// ```
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Package {
	action: Action,
	data: Vec<u8>,
	password_hash: Option<[u8; 32]>,
}

impl Package {
	crate::accessor!(copy action -> Action);

	crate::accessor!(& data -> &[u8]);

	#[must_use = "Send a package with `self.send`."]
	pub fn new<D>(password: Option<&str>, action: Action, data: D) -> Self
	where
		D: Into<Vec<u8>>,
	{
		let mut rv = Self { password_hash: None, action, data: data.into() };
		rv.set_password(password);
		rv
	}

	/// Receives bytes from a [`stream`](tokio::net::TcpStream) with timeout
	/// `consts::PACKAGE_RECEIVE_TIMEOUT` and deserializes into the
	/// structure. Also makes sure that the `self.action` field is in
	/// `accepted_actions`.
	///
	/// See also: [`send`](Package::send).
	pub async fn receive(
		stream: &mut tokio::net::TcpStream,
		password: Option<&str>,
		accepted_actions: Option<std::collections::HashSet<Action>>,
	) -> Result<Self, ReceivePackageError> {
		let bytes = Self::receive_bytes(stream).await?;
		let package: Self = bincode::deserialize(&bytes)?;
		if !package.check_password(password) {
			Package::new(None, Action::InvalidPassword, vec![])
				.send(stream)
				.await?;
			return Err(ReceivePackageError::InvalidPassword);
		}
		if let Some(aa) = accepted_actions {
			if !aa.contains(&package.action) {
				return Err(ReceivePackageError::InvalidAction);
			}
		}
		Ok(package)
	}

	/// Receives bytes from a [`stream`](tokio::net::TcpStream) with timeouts
	/// `consts::PACKAGE_RECEIVE_TIMEOUT`
	///
	/// See also: [`send`](Package::send).
	///
	/// `#[allow(clippy::needless_pass_by_ref_mut)]` because of clippy bug.
	#[allow(clippy::needless_pass_by_ref_mut)]
	async fn receive_bytes(
		stream: &mut tokio::net::TcpStream,
	) -> Result<Box<[u8]>, ReceivePackageBytesError> {
		tokio::time::timeout(crate::consts::PACKAGE_RECEIVE_TIMEOUT, async {
			use tokio::io::AsyncReadExt as _;

			// Receive a size
			let mut size_be_bytes_buffer = [0; 8];
			stream
				.read_exact(&mut size_be_bytes_buffer)
				.await
				.map_err(ReceivePackageBytesError::ReceiveSize)?;
			let size = usize::from_be_bytes(size_be_bytes_buffer);
			if size > crate::consts::PACKAGE_MAX_SIZE {
				return Err(ReceivePackageBytesError::TooBig);
			}
			// Receive a bytes
			let mut bytes_buffer = vec![0; size].into_boxed_slice();
			stream
				.read_exact(&mut bytes_buffer)
				.await
				.map_err(ReceivePackageBytesError::ReceiveData)?;
			Ok(bytes_buffer)
		})
		.await?
	}

	/// `true` if [serialized size](bincode::serialized_size) of `self` greater
	/// than `consts::PACKAGE_MAX_SIZE`, else `false`.
	pub fn is_too_big(&self) -> Result<bool, PackageIsTooBigError> {
		let size = bincode::serialized_size(self)?;
		Ok(size > crate::consts::PACKAGE_MAX_SIZE as u64)
	}

	pub fn set_password(&mut self, password: Option<&str>) {
		self.password_hash = password.map(|p| {
			crate::crypto::hash_with_salt(p, crate::consts::PASSWORD_SALT)
		});
	}

	#[must_use]
	pub fn check_password(&self, password: Option<&str>) -> bool {
		self.password_hash
			== password.map(|p| {
				crate::crypto::hash_with_salt(p, crate::consts::PASSWORD_SALT)
			})
	}

	/// Sends `self` to [`stream`](tokio::net::TcpStream).
	///
	/// First it sends a data with a size of 8 bytes, which contains the size
	/// of the `self`. Then it sends the `self` bytes.
	pub async fn send(
		&self,
		stream: &mut tokio::net::TcpStream,
	) -> Result<(), SendPackageError> {
		use tokio::io::AsyncWriteExt as _;

		let data = bincode::serialize(self)?;
		if data.len() > crate::consts::PACKAGE_MAX_SIZE {
			return Err(SendPackageError::TooBig);
		}
		let size_u64_be_bytes = (data.len() as u64).to_be_bytes();
		stream
			.write_all(&size_u64_be_bytes)
			.await
			.map_err(SendPackageError::SendSize)?;
		stream.write_all(&data).await.map_err(SendPackageError::SendData)?;
		Ok(())
	}
}
