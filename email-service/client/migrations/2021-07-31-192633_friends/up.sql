-- # Explanation of some fields
--
-- aes key = sha256(current user password, current user username)
--
-- `.username_hash` = sha256(friend username, current user salt)
-- `.public_key_pem_base64_hash`
-- = sha256(friend public key pem base64, current user salt)
-- `.encrypted_username` = aes[aes key](friend username)
-- `.encrypted_public_key_pem_base64` = aes[aes key](friend public key pem base64)
CREATE TABLE friends (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	username_hash BYTEA NOT NULL UNIQUE,
	public_key_pem_base64_hash BYTEA NOT NULL UNIQUE,
	encrypted_username BYTEA NOT NULL UNIQUE,
	encrypted_public_key_pem_base64 BYTEA NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
