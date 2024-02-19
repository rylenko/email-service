// Use `crate` as `common` to call macros
use crate::{
	self as common,
	error::{
		CheckEmailDecryptedIntegrityError, CheckEmailSignatureError,
		DecryptEmailError, NewEmailError, SignEmailError,
	},
};

/// The structure that represents in HTML the file that was attached to the
/// email.
///
/// `data` is encoded into [`base64`].
#[derive(serde::Deserialize, serde::Serialize)]
pub struct File {
	name: String,
	data: String,
}

impl File {
	#[must_use]
	pub fn new<N: Into<String>, D: AsRef<[u8]>>(name: N, data: D) -> Self {
		Self { name: name.into(), data: base64::encode(data) }
	}
}

/// The structure that represents the data of the email to be shown to the
/// recipient in HTML.
#[derive(serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Data {
	sender_username: String,
	title: String,
	text: String,
	files: Option<Vec<File>>,
	#[serde(with = "chrono::serde::ts_seconds")]
	sent_at: chrono::DateTime<chrono::Utc>,
}

impl Data {
	common::accessor!(& sender_username -> &str);

	#[inline]
	#[must_use]
	pub fn new(
		sender_username: String,
		title: String,
		text: String,
		files: Option<Vec<File>>,
	) -> Self {
		Self {
			sender_username,
			title,
			text,
			files,
			sent_at: chrono::Utc::now(),
		}
	}
}

/// An email that is transmitted between client and node.
///
/// # Examples
///
/// Creating:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// # let recipient_private_key = openssl::rsa::Rsa::generate(2048)?;
/// # let recipient_public_key_pem
/// #     = recipient_private_key.public_key_to_pem()?;
/// # let recipient_public_key
/// #     = openssl::rsa::Rsa::public_key_from_pem(&recipient_public_key_pem)?;
/// # let sender_private_key = openssl::rsa::Rsa::generate(2048)?;
/// let data = common::email::Data::new(
///     "sender".to_owned(), "title".to_owned(), "text".to_owned(), None,
/// );
/// let mut email = common::email::Email::new(&recipient_public_key, data)?;
/// email = tokio::task::spawn_blocking(move || {
///     email.generate_proof_of_work();
///     email
/// }).await?;
/// email.sign(&sender_private_key)?;
/// # let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8888").await?;
/// # let package = common::package::Package::new(
/// #     None,
/// #     common::package::Action::SendEmail,
/// #     bincode::serialize(&email)?,
/// # );
/// # package.send(&mut stream).await?;
/// # Ok(())
/// # }
/// ```
///
/// Receiving (without decrypting):
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// # let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8888").await?;
/// # let package = common::package::Package::receive(
/// #     &mut stream,
/// #     None,
/// #     Some(common::set![common::package::Action::SendEmail]),
/// # ).await?;
/// let email = bincode::deserialize::<common::email::Email>(package.data())?;
/// assert!(email.check_encrypted_integrity(), "Invalid email.");
/// # Ok(())
/// # }
/// ```
///
/// Receiving (with decrypting):
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// # let private_key = openssl::rsa::Rsa::generate(2048)?;
/// # let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8888").await?;
/// # let package = common::package::Package::receive(
/// #     &mut stream,
/// #     None,
/// #    Some(common::set![common::package::Action::SendEmail]),
/// # ).await?;
/// let mut email
///     = bincode::deserialize::<common::email::Email>(package.data())?;
/// if !email.check_encrypted_integrity()
///     || email.decrypt(&private_key).is_err()
///     || !email.check_decrypted_integrity()?
/// {
///     panic!("Invalid email.");
/// }
/// println!("Sender: {}", email.data().unwrap().sender_username());
/// # Ok(())
/// # }
/// ```
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Email {
	recipient_public_key_pem_hash: [u8; 32],
	nonce: u64,
	#[serde(skip)]
	session: Option<Box<[u8]>>,
	e_session: Box<[u8]>,
	#[serde(skip)]
	data: Option<Data>,
	e_data_bytes: Box<[u8]>,
	#[serde(skip)]
	sender_public_key_pem: Option<Box<[u8]>>,
	e_sender_public_key_pem: Option<Box<[u8]>>,
	#[serde(skip)]
	signature: Option<Box<[u8]>>,
	e_signature: Option<Box<[u8]>>,
}

impl Email {
	crate::accessor!(& recipient_public_key_pem_hash -> &[u8; 32]);

	crate::accessor!(as_ref data -> Option<&Data>);

	crate::accessor!(as_deref sender_public_key_pem -> Option<&[u8]>);

	pub fn new(
		recipient_public_key: &openssl::rsa::Rsa<openssl::pkey::Public>,
		data: Data,
	) -> Result<Self, NewEmailError> {
		crate::debug!("Creating a new email...");

		// Get recipient's public key pem hash
		let recipient_public_key_pem = recipient_public_key
			.public_key_to_pem()
			.map_err(NewEmailError::RecipientPublicKeyToPem)?;
		let recipient_public_key_pem_hash =
			crate::crypto::hash(recipient_public_key_pem);

		// Generate and encrypt session using public key and PKCS1_OAEP padding
		let session =
			crate::crypto::generate_random_bytes(None)?.into_boxed_slice();
		let mut e_session =
			vec![0; recipient_public_key.size() as usize].into_boxed_slice();
		recipient_public_key
			.public_encrypt(
				&session,
				&mut e_session,
				openssl::rsa::Padding::PKCS1_OAEP,
			)
			.map_err(NewEmailError::EncryptSession)?;

		// Serialize and encrypt a data using session
		let data_bytes = bincode::serialize(&data)?;
		let e_data_bytes = crate::crypto::AesCipher::new(&*session)
			.encrypt(data_bytes)?
			.into_boxed_slice();

		Ok(Self {
			recipient_public_key_pem_hash,
			e_session,
			session: Some(session),
			data: Some(data),
			e_data_bytes,
			..Self::default()
		})
	}

	/// Generates a proof-of-work.
	///
	/// May take a long time. It is better to use this in conjunction with
	/// [`tokio::task::spawn_blocking`].
	pub fn generate_proof_of_work(&mut self) {
		crate::debug!("Generation of proof-of-work in the email...");
		while !self.check_proof_of_work() {
			self.nonce += 1;
		}
	}

	/// Signs the proof-of-work with the
	/// [`private_key`](openssl::rsa::Rsa<Private>).
	///
	/// # Panics
	///
	/// If you are not sender and have not used [`decrypt`](Email::decrypt).
	pub fn sign(
		&mut self,
		private_key: &openssl::rsa::Rsa<openssl::pkey::Private>,
	) -> Result<(), SignEmailError> {
		crate::debug!("Signing a email...");

		// Create signer
		let pkey = openssl::pkey::PKey::from_rsa(private_key.clone())
			.map_err(SignEmailError::PkeyFromRsa)?;
		let mut signer = openssl::sign::Signer::new(
			openssl::hash::MessageDigest::sha256(),
			&pkey,
		)
		.map_err(SignEmailError::NewSigner)?;
		signer
			.set_rsa_padding(openssl::rsa::Padding::PKCS1_PSS)
			.map_err(SignEmailError::SetPadding)?;
		signer
			.update(self.compute_hash().as_bytes())
			.map_err(SignEmailError::UpdateSigner)?;

		// Sign, encrypt signature and sender public key
		let signature = signer
			.sign_to_vec()
			.map_err(SignEmailError::Sign)?
			.into_boxed_slice();
		let sender_public_key_pem = private_key
			.public_key_to_pem()
			.map_err(SignEmailError::PublicKeyToPem)?
			.into_boxed_slice();
		let cipher = self.make_aes_cipher();
		let e_signature = cipher
			.encrypt(&signature)
			.map_err(SignEmailError::EncryptSignature)?
			.into_boxed_slice();
		let e_sender_public_key_pem = cipher
			.encrypt(&sender_public_key_pem)
			.map_err(SignEmailError::EncryptPublicKeyPem)?
			.into_boxed_slice();

		// Update fields
		self.e_signature = Some(e_signature);
		self.signature = Some(signature);
		self.e_sender_public_key_pem = Some(e_sender_public_key_pem);
		self.sender_public_key_pem = Some(sender_public_key_pem);
		Ok(())
	}

	/// Checks the proof-of-work and that email is signed. If you want to check
	/// signature, use
	/// [`check_decrypted_integrity`](Email::check_decrypted_integrity).
	#[must_use]
	pub fn check_encrypted_integrity(&self) -> bool {
		if !self.check_proof_of_work() {
			crate::debug!("Encrypted email has invalid proof-of-work.");
			return false;
		} else if !self.check_is_signed() {
			crate::debug!("Encrypted email is not signed.");
			return false;
		}
		true
	}

	/// Checks a decrypted signature.
	///
	/// # Panics
	///
	/// 1. If you are not sender and have not used [`decrypt`](Email::decrypt).
	/// 2. If you are sender and have not used [`sign`](Email::sign).
	pub fn check_decrypted_integrity(
		&self,
	) -> Result<bool, CheckEmailDecryptedIntegrityError> {
		if !self.check_signature()? {
			crate::debug!("Encrypted email has invalid signature.");
			return Ok(false);
		}
		Ok(true)
	}

	/// With the [`private_key`](openssl::rsa::Rsa<openssl::pkey::Private>)
	/// it decrypts encrypted session, then using decrypted session decrypts
	/// encrypted data, sender public key and sender signature.
	///
	/// # Panics
	///
	/// If email is not [signed](Email::sign)ed. Use
	/// [`check_encrypted_integrity`](Email::decrypt) before.
	pub fn decrypt(
		&mut self,
		private_key: &openssl::rsa::Rsa<openssl::pkey::Private>,
	) -> Result<(), DecryptEmailError> {
		// Decrypt session
		let mut session = vec![0; private_key.size() as usize];
		private_key.private_decrypt(
			&self.e_session,
			&mut session,
			openssl::rsa::Padding::PKCS1_OAEP,
		)?;
		session.truncate(crate::consts::DEFAULT_RANDOM_BYTES_LENGTH);
		self.session = Some(session.into_boxed_slice());

		// Decrypt other fields
		let cipher = self.make_aes_cipher();
		let data_bytes = cipher
			.decrypt(&self.e_data_bytes)
			.map_err(DecryptEmailError::DataBytes)?;
		let sender_public_key_pem = cipher
			.decrypt(self.e_sender_public_key_pem.as_ref().unwrap())
			.map_err(DecryptEmailError::SenderPublicKeyPem)?;
		let signature = cipher
			.decrypt(self.e_signature.as_ref().unwrap())
			.map_err(DecryptEmailError::Signature)?;

		// Update fields
		self.data = Some(bincode::deserialize(&data_bytes)?);
		self.sender_public_key_pem =
			Some(sender_public_key_pem.into_boxed_slice());
		self.signature = Some(signature.into_boxed_slice());
		Ok(())
	}

	/// Calculates the current hash of the email.
	///
	/// Doesn't take some fields, as they only make sense after the proof of
	/// work has been generated.
	#[must_use]
	pub fn compute_hash(&self) -> String {
		let parts = [
			&self.nonce.to_be_bytes(),
			&*self.e_session,
			&self.recipient_public_key_pem_hash,
			&self.e_data_bytes,
		];
		hex::encode(crate::crypto::hash(parts.concat()))
	}

	/// Checks that the encrypted signature and encrypted sender public key
	/// fields are [`Some`].
	#[must_use]
	fn check_is_signed(&self) -> bool {
		self.e_signature.is_some() && self.e_sender_public_key_pem.is_some()
	}

	/// Checks that the [`compute_hash`](Email::compute_hash) starts
	/// with `consts::PROOF_OF_WORK_DIFFICULTY_STRING`.
	#[must_use]
	fn check_proof_of_work(&self) -> bool {
		self.compute_hash()
			.starts_with(&*crate::consts::PROOF_OF_WORK_DIFFICULTY_STRING)
	}

	/// Checks that the sender has [signed](Email::sign)
	/// [`compute_hash`](Email::compute_hash) with his
	/// private key with [PKCS1-PSS padding](openssl::rsa::Padding).
	///
	/// # Panics
	///
	/// 1. If you are not sender and have not used [`decrypt`](Email::decrypt).
	/// 2. If you are sender and have not used [`sign`](Email::sign).
	fn check_signature(&self) -> Result<bool, CheckEmailSignatureError> {
		if !self.check_is_signed() {
			return Ok(false);
		}
		// Convert sender pem to `PKey`
		let sender_public_key = openssl::rsa::Rsa::public_key_from_pem(
			self.sender_public_key_pem.as_ref().unwrap(),
		)
		.map_err(CheckEmailSignatureError::PublicKeyFromPem)?;
		let pkey = openssl::pkey::PKey::from_rsa(sender_public_key)
			.map_err(CheckEmailSignatureError::PkeyFromRsa)?;

		// Create, update verifier and verify
		let mut verifier = openssl::sign::Verifier::new(
			openssl::hash::MessageDigest::sha256(),
			&pkey,
		)
		.map_err(CheckEmailSignatureError::NewVerifier)?;
		verifier
			.set_rsa_padding(openssl::rsa::Padding::PKCS1_PSS)
			.map_err(CheckEmailSignatureError::SetPadding)?;
		verifier
			.update(self.compute_hash().as_bytes())
			.map_err(CheckEmailSignatureError::UpdateVerifier)?;
		let rv = verifier
			.verify(self.signature.as_ref().unwrap())
			.map_err(CheckEmailSignatureError::Verify)?;
		Ok(rv)
	}

	/// Creates `crypto::AesCipher` with key `self.session`.
	///
	/// # Panics
	///
	/// If you are not sender and have not used [`decrypt`](Email::decrypt).
	#[must_use]
	fn make_aes_cipher(&self) -> crate::crypto::AesCipher {
		crate::crypto::AesCipher::new(self.session.as_ref().unwrap().as_ref())
	}
}
