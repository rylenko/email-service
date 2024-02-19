-- # Explanation of some fields
--
-- `.recipient_public_key_pem_hash` - Duplicate of field from `Email` object.
-- Needed for easy search.
-- `.proof_of_work` - proof of work from `Email`. Needed to avoid duplicates.
CREATE TABLE emails (
	id SERIAL PRIMARY KEY,
	email_bytes BYTEA NOT NULL,
	recipient_public_key_pem_hash BYTEA NOT NULL,
	proof_of_work VARCHAR(64) NOT NULL UNIQUE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
