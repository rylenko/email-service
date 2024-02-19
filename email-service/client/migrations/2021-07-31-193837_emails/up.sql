-- # Explanation of some fields
--
-- aes key = sha256(current user password, current user username)
--
-- `.encrypted_sender_public_key_pem` = aes[aes key](sender public key pem)
-- `.encrypted_data_bytes` = aes[aes key](common::email::Data bytes)
-- `.proof_of_work` - proof of work from `common::email::Email`.
-- Needed to avoid duplicate packages
CREATE TABLE emails (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	encrypted_sender_public_key_pem BYTEA NOT NULL,
	encrypted_data_bytes BYTEA NOT NULL,
	proof_of_work VARCHAR(64) NOT NULL UNIQUE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
