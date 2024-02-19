-- # Explanation of some fields
--
-- aes key = sha256(current user password, current user username)
--
-- `.address_hash` = sha256(address, current user salt)
-- `.encrypted_address` = aes[aes key](address)
-- `.encrypted_password` = aes[aes key](password)
CREATE TABLE nodes (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	address_hash BYTEA NOT NULL UNIQUE,
	encrypted_address BYTEA NOT NULL UNIQUE,
	encrypted_password BYTEA,
	created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
