#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AesDecryptError {
	#[error("Failed to decrypt.")]
	Decrypt(#[from] openssl::error::ErrorStack),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AesDecryptBase64Error {
	#[error("Failed to decrypt.")]
	Decrypt(#[from] AesDecryptError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AesDecryptStringError {
	#[error("Failed to build String from UTF-8.")]
	FromUtf8(#[from] std::string::FromUtf8Error),
	#[error("Failed to decrypt.")]
	Decrypt(#[from] AesDecryptError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AesEncryptError {
	#[error("Failed to generate iv.")]
	GenerateIv(#[from] GenerateRandomBytesError),
	#[error("Failed to encrypt.")]
	Encrypt(#[from] openssl::error::ErrorStack),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CheckEmailDecryptedIntegrityError {
	#[error("Failed to check email signature.")]
	Signature(#[from] CheckEmailSignatureError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CheckEmailSignatureError {
	#[error("Failed to create a new verifier.")]
	NewVerifier(#[source] openssl::error::ErrorStack),
	#[error("Failed to get pkey from rsa private key.")]
	PkeyFromRsa(#[source] openssl::error::ErrorStack),
	#[error("Failed to convert PEM to public key.")]
	PublicKeyFromPem(#[source] openssl::error::ErrorStack),
	#[error("Failed to set the padding.")]
	SetPadding(#[source] openssl::error::ErrorStack),
	#[error("Failed to verify a signature.")]
	Verify(#[source] openssl::error::ErrorStack),
	#[error("Failed to update a verifier.")]
	UpdateVerifier(#[source] openssl::error::ErrorStack),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CreateDbPoolError {
	#[error("Failed to build.")]
	Build(#[from] diesel_async::pooled_connection::deadpool::BuildError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DecryptEmailError {
	#[error("Failed to convert bytes to data.")]
	DataFromBytes(#[from] bincode::Error),
	#[error("Failed to decrypt a data bytes.")]
	DataBytes(#[source] AesDecryptError),
	#[error("Failed to decrypt a sender public key PEM.")]
	SenderPublicKeyPem(#[source] AesDecryptError),
	#[error("Failed to decrypt a session.")]
	Session(#[from] openssl::error::ErrorStack),
	#[error("Failed to decrypt a signature.")]
	Signature(#[source] AesDecryptError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DeserializeJsonFromFileError {
	#[error("Failed to read a file.")]
	Read(#[from] std::io::Error),
	#[error("Failed to deserialize.")]
	Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GenerateRandomBytesError {
	#[error(transparent)]
	Generate(#[from] getrandom::Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NewEmailError {
	#[error("Failed to convert data to bytes.")]
	DataToBytes(#[from] bincode::Error),
	#[error("Failed to encrypt data bytes.")]
	EncryptDataBytes(#[from] AesEncryptError),
	#[error("Failed to encrypt a session.")]
	EncryptSession(#[source] openssl::error::ErrorStack),
	#[error("Failed to generate a session.")]
	GenerateSession(#[from] GenerateRandomBytesError),
	#[error("Failed to convert recipient's public key to PEM.")]
	RecipientPublicKeyToPem(#[source] openssl::error::ErrorStack),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PackageIsTooBigError {
	#[error("Failed to get serialized size.")]
	SerializedSize(#[from] bincode::Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReceivePackageBytesError {
	#[error("Timeout.")]
	Elapsed(#[from] tokio::time::error::Elapsed),
	#[error("Failed to receive a file.")]
	ReceiveData(#[source] std::io::Error),
	#[error("Failed to receive a size.")]
	ReceiveSize(#[source] std::io::Error),
	#[error("Too big.")]
	TooBig,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReceivePackageError {
	#[error("Failed to build package from bytes.")]
	FromBytes(#[from] bincode::Error),
	#[error("Invalid action.")]
	InvalidAction,
	#[error("Invalid password.")]
	InvalidPassword,
	#[error("Failed to receive package's bytes.")]
	ReceiveBytes(#[from] ReceivePackageBytesError),
	#[error("Failed to send a package.")]
	SendPackage(#[from] SendPackageError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SendEmailToNodesError {
	#[error("Failed to join a task.")]
	JoinTask(#[from] tokio::task::JoinError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SendPackageError {
	#[error("Failed to send a data.")]
	SendData(#[source] std::io::Error),
	#[error("Failed to send a size.")]
	SendSize(#[source] std::io::Error),
	#[error("Failed to convert package to bytes.")]
	ToBytes(#[from] bincode::Error),
	#[error("Package is too big.")]
	TooBig,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SignEmailError {
	#[error("Failed to encrypt a public key pem.")]
	EncryptPublicKeyPem(#[source] AesEncryptError),
	#[error("Failed to encrypt a signature.")]
	EncryptSignature(#[source] AesEncryptError),
	#[error("Failed to create a new signer.")]
	NewSigner(#[source] openssl::error::ErrorStack),
	#[error("Failed to get pkey from rsa private key.")]
	PkeyFromRsa(#[source] openssl::error::ErrorStack),
	#[error("Failed to convert public key to PEM.")]
	PublicKeyToPem(#[source] openssl::error::ErrorStack),
	#[error("Failed to set the padding.")]
	SetPadding(#[source] openssl::error::ErrorStack),
	#[error("Failed to sign using signer.")]
	Sign(#[source] openssl::error::ErrorStack),
	#[error("Failed to update a signer.")]
	UpdateSigner(#[source] openssl::error::ErrorStack),
}
