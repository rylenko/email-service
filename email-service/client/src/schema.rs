// @generated automatically by Diesel CLI.

diesel::table! {
	emails (id) {
		id -> Int4,
		user_id -> Int4,
		encrypted_sender_public_key_pem -> Bytea,
		encrypted_data_bytes -> Bytea,
		proof_of_work -> Varchar,
		created_at -> Timestamp,
	}
}

diesel::table! {
	friends (id) {
		id -> Int4,
		user_id -> Int4,
		username_hash -> Bytea,
		public_key_pem_base64_hash -> Bytea,
		encrypted_username -> Bytea,
		encrypted_public_key_pem_base64 -> Bytea,
		created_at -> Timestamp,
	}
}

diesel::table! {
	nodes (id) {
		id -> Int4,
		user_id -> Int4,
		address_hash -> Bytea,
		encrypted_address -> Bytea,
		encrypted_password -> Nullable<Bytea>,
		created_at -> Timestamp,
	}
}

diesel::table! {
	users (id) {
		id -> Int4,
		username_hash -> Bytea,
		password_hash -> Bytea,
		encrypted_private_key_pem -> Bytea,
		salt -> Bytea,
		f2f_enabled -> Bool,
		created_at -> Timestamp,
	}
}

diesel::joinable!(emails -> users (user_id));
diesel::joinable!(friends -> users (user_id));
diesel::joinable!(nodes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(emails, friends, nodes, users,);
