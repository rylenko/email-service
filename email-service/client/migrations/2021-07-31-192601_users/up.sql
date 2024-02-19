-- # Explanation of some fields
--
-- aes key = sha256(current user password, current user username)
--
-- `.username_hash` = sha256(current user username)
-- `.password_hash` = sha256(current user password, current user salt)
-- `.encrypted_private_key_pem` = aes[aes key](private key pem)
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	username_hash BYTEA NOT NULL UNIQUE,
	password_hash BYTEA NOT NULL,
	-- Do not use `UNIQUE` here, since the values are too large to control uniqueness.
	encrypted_private_key_pem BYTEA NOT NULL,
	salt BYTEA NOT NULL,
	f2f_enabled BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
