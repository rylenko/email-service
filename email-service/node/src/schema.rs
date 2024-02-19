// @generated automatically by Diesel CLI.

diesel::table! {
	emails (id) {
		id -> Int4,
		email_bytes -> Bytea,
		recipient_public_key_pem_hash -> Bytea,
		proof_of_work -> Varchar,
		created_at -> Timestamp,
	}
}
