use super::error::{
	AddFriendGetError, AddFriendPostError, AddNodeGetError, AddNodePostError,
	DeleteAccountGetError, DeleteAccountPostError, DeleteFriendError,
	DeleteNodeError, EmailError, EmailsError, FriendsError, IndexError,
	LoadEmailsError, LoginGetError, LoginPostError, LogoutError,
	NodesGetError, NodesPostError, ProfileError, RegisterGetError,
	RegisterPostError, SendEmailGetError, SendEmailPostError, SwitchF2fError,
};

macro_rules! validate_at_least_one_friend_and_one_node {
	($r:expr, $friends:expr, $nodes:expr, $error:ty $(,)?) => {
		if $friends.is_empty() {
			super::flash::add($r, "Add at least one friend.", "warning")
				.map_err(<$error>::AddFriendFlash)?;
			return super::response::redirect_static($r, "add_friend_get")
				.map_err(<$error>::AddFriendRedirectStatic);
		} else if $nodes.is_empty() {
			super::flash::add($r, "Add at least one node.", "warning")
				.map_err(<$error>::AddNodeFlash)?;
			return super::response::redirect_static($r, "add_node_get")
				.map_err(<$error>::AddNodeRedirectStatic);
		}
	};
}

#[actix_web::get("/friends/add/")]
pub(crate) async fn add_friend_get(
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, AddFriendGetError> {
	super::auth::validate_logged_in(&r)?;
	Ok(super::response::render(
		&r,
		"add-friend.html",
		None,
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::post("/friends/add/")]
pub(crate) async fn add_friend_post(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	form: actix_web::web::Form<super::forms::Friend>,
) -> Result<actix_web::HttpResponse, AddFriendPostError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_in(&r)?;

	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();
	if let Err(ref errors) =
		form.validate_args(((s.db(), &user), (s.db(), &user), &r))
	{
		return Ok(super::response::render_form_errors(
			&r,
			"add-friend.html",
			errors,
			None,
		)?);
	}
	s.db()
		.add_friend(&user, &form.username, &form.public_key_pem_base64)
		.await?;
	super::flash::add(
		&r,
		"Your friend has been successfully added.",
		"success",
	)?;
	Ok(super::response::redirect_static(&r, "friends")?)
}

#[actix_web::get("/nodes/add/")]
pub(crate) async fn add_node_get(
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, AddNodeGetError> {
	super::auth::validate_logged_in(&r)?;
	Ok(super::response::render(
		&r,
		"add-node.html",
		None,
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::post("/nodes/add/")]
pub(crate) async fn add_node_post(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	form: actix_web::web::Form<super::forms::Node>,
) -> Result<actix_web::HttpResponse, AddNodePostError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_in(&r)?;

	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();
	if let Err(ref errors) = form.validate_args(((s.db(), &user), &r)) {
		return Ok(super::response::render_form_errors(
			&r,
			"add-node.html",
			errors,
			None,
		)?);
	}
	let password = if form.password.is_empty() {
		None
	} else {
		Some(form.password.as_str())
	};
	s.db().add_node(&user, &form.address, password).await?;
	super::flash::add(
		&r,
		"Your node has been successfully added.",
		"success",
	)?;
	Ok(super::response::redirect_static(&r, "nodes_get")?)
}

#[actix_web::get("/delete-account/")]
pub(crate) async fn delete_account_get(
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, DeleteAccountGetError> {
	super::auth::validate_logged_in(&r)?;
	Ok(super::response::render(
		&r,
		"delete-account.html",
		None,
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::post("/delete-account/")]
pub(crate) async fn delete_account_post(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	identity: actix_identity::Identity,
	form: actix_web::web::Form<super::forms::DeleteAccount>,
) -> Result<actix_web::HttpResponse, DeleteAccountPostError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_in(&r)?;

	// Get user and validate the form
	//
	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();
	if let Err(ref errors) = form.validate_args((&user, &r)) {
		return Ok(super::response::render_form_errors(
			&r,
			"delete-account.html",
			errors,
			None,
		)?);
	}

	// Delete user from the database and from the identity
	s.db().delete_user(&user).await?;
	identity.logout();

	// Flash the message and redirect
	super::flash::add(&r, "You have deleted your account.", "danger")?;
	Ok(super::response::redirect_static(&r, "index")?)
}

#[actix_web::post("/friends/{id}/delete/")]
pub(crate) async fn delete_friend(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	id: actix_web::web::Path<i32>,
	form: actix_web::web::Form<super::forms::Csrf>,
) -> Result<actix_web::HttpResponse, DeleteFriendError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_in(&r)?;

	match form.validate_args(&r) {
		Ok(()) => {
			// We can use `Option::unwrap` because of
			// `super::auth::validate_logged_in`
			let user = super::auth::get_current_user(&r)?.unwrap();
			s.db().delete_friend(&user, *id).await?;
			super::flash::add(&r, "You have deleted your friend.", "danger")?;
		}
		Err(ref errors) => super::flash::add_form_errors(&r, errors)?,
	}
	Ok(super::response::redirect_static(&r, "friends")?)
}

#[actix_web::post("/nodes/{id}/delete/")]
pub(crate) async fn delete_node(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	id: actix_web::web::Path<i32>,
	form: actix_web::web::Form<super::forms::Csrf>,
) -> Result<actix_web::HttpResponse, DeleteNodeError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_in(&r)?;

	match form.validate_args(&r) {
		Ok(()) => {
			// We can use `Option::unwrap` because of
			// `super::auth::validate_logged_in`
			let user = super::auth::get_current_user(&r)?.unwrap();
			s.db().delete_node(&user, *id).await?;
			super::flash::add(&r, "You have deleted your node.", "danger")?;
		}
		Err(ref errors) => super::flash::add_form_errors(&r, errors)?,
	}
	Ok(super::response::redirect_static(&r, "nodes_get")?)
}

#[actix_web::get("/emails/{id}/")]
pub(crate) async fn email(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	id: actix_web::web::Path<i32>,
) -> Result<actix_web::HttpResponse, EmailError> {
	super::auth::validate_logged_in(&r)?;

	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();
	let email_ =
		s.db().get_email(&user, *id).await.map_err(EmailError::GetEmail)?;
	let friend_same_public_key =
		match s.db().get_friend(&user, email_.sender_public_key()).await {
			Ok(f) => Some(f),
			Err(ref e) if super::error::check_diesel_not_found_down(e) => None,
			Err(e) => return Err(EmailError::GetFriend(e)),
		};
	let friend_same_username_exists = s
		.db()
		.check_friend_exists_by_username(
			&user,
			email_.data().sender_username(),
		)
		.await
		.map_err(EmailError::CheckFriendExistsByUsername)?;

	let context = context! {
		"email" => &email_,
		"friend_same_public_key" => &friend_same_public_key,
		"friend_same_username_exists" => &friend_same_username_exists,
	};
	Ok(super::response::render(
		&r,
		"email.html",
		Some(context),
		actix_web::http::StatusCode::OK,
		false,
	)?)
}

#[actix_web::get("/emails/")]
pub(crate) async fn emails(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	query: actix_web::web::Query<super::pagination::Query>,
) -> Result<actix_web::HttpResponse, EmailsError> {
	super::auth::validate_logged_in(&r)?;
	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();

	// Get emails with pagination
	let page = query
		.page()
		.unwrap_or(unsafe { std::num::NonZeroU64::new_unchecked(1) });
	let pagination = s.db().get_emails(&user, page).await?;
	// On the first page, if there are no items,  we can place a corresponding
	// inscription.
	if pagination.items().is_empty() && u64::from(page) > 1 {
		return Err(EmailsError::InvalidPage);
	}

	let context = context! {
		"pagination" => &pagination,
		"emails_max_age_secs" => &common::consts::EMAILS_MAX_AGE.as_secs(),
	};
	Ok(super::response::render(
		&r,
		"emails.html",
		Some(context),
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::get("/friends/")]
pub(crate) async fn friends(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, FriendsError> {
	super::auth::validate_logged_in(&r)?;

	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();
	let friends = s.db().get_friends(&user).await?;

	let context = context! {"friends" => &friends};
	Ok(super::response::render(
		&r,
		"friends.html",
		Some(context),
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::get("/")]
pub(crate) async fn index(
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, IndexError> {
	Ok(super::response::render(
		&r,
		"index.html",
		None,
		actix_web::http::StatusCode::OK,
		false,
	)?)
}

#[actix_web::post("/emails/load/")]
pub(crate) async fn load_emails(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	form: actix_web::web::Form<super::forms::Csrf>,
) -> Result<actix_web::HttpResponse, LoadEmailsError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_in(&r)?;

	// Validate the form
	if let Err(ref errors) = form.validate_args(&r) {
		super::flash::add_form_errors(&r, errors)?;
		return super::response::redirect_static(&r, "emails")
			.map_err(LoadEmailsError::ValidationRedirectStatic);
	}

	// Get current user, his nodes and private key
	//
	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user =
		std::sync::Arc::new(super::auth::get_current_user(&r)?.unwrap());
	let private_key = std::sync::Arc::new(
		s.db()
			.get_user_private_key(&user)
			.await
			.map_err(LoadEmailsError::GetUserPrivateKey)?,
	);
	let nodes =
		s.db().get_nodes(&user).await.map_err(LoadEmailsError::GetNodes)?;

	// Spawn load futures
	let mut futures = Vec::with_capacity(nodes.len());
	for node in nodes {
		futures.push(tokio::spawn(super::request_node::load_emails(
			node,
			s.clone(),
			user.clone(),
			private_key.clone(),
		)));
	}

	// Sum, flash added emails count and redirect
	let mut added_count: usize = 0;
	for rr in futures::future::join_all(futures).await {
		added_count += rr?? as usize;
	}
	let message = format!("New emails loaded: {added_count}");
	common::debug!(message);
	super::flash::add(&r, &message, "success")?;
	super::response::redirect_static(&r, "emails")
		.map_err(LoadEmailsError::SuccessRedirectStatic)
}

#[actix_web::get("/login/")]
pub(crate) async fn login_get(
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, LoginGetError> {
	super::auth::validate_logged_out(&r)?;
	Ok(super::response::render(
		&r,
		"login.html",
		None,
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::post("/login/")]
pub(crate) async fn login_post(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	form: actix_web::web::Form<super::forms::Login>,
) -> Result<actix_web::HttpResponse, LoginPostError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_out(&r)?;

	// Validate the form
	if let Err(ref errors) = form.validate_args(&r) {
		return Ok(super::response::render_form_errors(
			&r,
			"login.html",
			errors,
			None,
		)?);
	}
	match s.db().get_user(form.username.clone(), form.password.clone()).await {
		Ok(u) => {
			// Login user, flash the message and redirect to index
			super::auth::login_user(&r, &u)?;
			super::flash::add(&r, "You are logged into account.", "success")?;
			Ok(super::response::redirect_static(&r, "index")?)
		}
		Err(ref e) if super::error::check_diesel_not_found_down(e) => {
			// Create an error, add it to the list and render the template
			let errors = validation_errors! {
				"invalid" => "Invalid username or password",
			};
			Ok(super::response::render_form_errors(
				&r,
				"login.html",
				&errors,
				None,
			)?)
		}
		Err(e) => Err(e.into()),
	}
}

#[actix_web::get("/logout/")]
pub(crate) async fn logout(
	r: actix_web::HttpRequest,
	identity: actix_identity::Identity,
) -> Result<actix_web::HttpResponse, LogoutError> {
	super::auth::validate_logged_in(&r)?;
	identity.logout();
	super::flash::add(&r, "You have logged out of account.", "success")?;
	Ok(super::response::redirect_static(&r, "index")?)
}

#[actix_web::get("/nodes/")]
pub(crate) async fn nodes_get(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, NodesGetError> {
	super::auth::validate_logged_in(&r)?;

	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();
	let nodes = s.db().get_nodes(&user).await?;

	let context = context! {"nodes" => &nodes};
	Ok(super::response::render(
		&r,
		"nodes.html",
		Some(context),
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::post("/nodes/")]
pub(crate) async fn nodes_post(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, NodesPostError> {
	super::auth::validate_logged_in(&r)?;

	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();
	let nodes = s.db().get_nodes(&user).await?;

	// Check connections
	let mut futures = Vec::with_capacity(nodes.len());
	for node in &nodes {
		futures.push(tokio::spawn(super::request_node::check_connection(
			s.clone(),
			node.clone(),
		)));
	}
	// Receive results from futures
	let mut results = Vec::with_capacity(futures.len());
	for rr in futures::future::join_all(futures).await {
		results.push(rr?);
	}

	let context =
		context! {"nodes" => &nodes, "check_connection_results" => &results};
	Ok(super::response::render(
		&r,
		"nodes.html",
		Some(context),
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::get("/profile/")]
pub(crate) async fn profile(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, ProfileError> {
	super::auth::validate_logged_in(&r)?;
	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();

	// Check F2F and get current user's private key
	let f2f_enabled = s
		.db()
		.check_user_f2f(&user)
		.await
		.map_err(ProfileError::CheckUserF2f)?;
	let private_key = s
		.db()
		.get_user_private_key(&user)
		.await
		.map_err(ProfileError::GetUserPrivateKey)?;

	// Get private and public key pems
	let private_key_pem = private_key.private_key_to_pem()?;
	let public_key_pem = private_key.public_key_to_pem()?;

	// Make QR codes
	let private_key_qrcode = super::qrcode::make_png_bytes(&private_key_pem);
	let public_key_qrcode = super::qrcode::make_png_bytes(&public_key_pem);

	let context = context! {
		"f2f_enabled" => &f2f_enabled,
		"private_key_pem_base64" => &base64::encode(private_key_pem),
		"public_key_pem_base64" => &base64::encode(public_key_pem),
		"private_key_qrcode" => &base64::encode(private_key_qrcode),
		"public_key_qrcode" => &base64::encode(public_key_qrcode),
	};
	Ok(super::response::render(
		&r,
		"profile.html",
		Some(context),
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::get("/register/")]
pub(crate) async fn register_get(
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, RegisterGetError> {
	super::auth::validate_logged_out(&r)?;
	Ok(super::response::render(
		&r,
		"register.html",
		None,
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::post("/register/")]
pub(crate) async fn register_post(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	form: actix_web::web::Form<super::forms::Register>,
) -> Result<actix_web::HttpResponse, RegisterPostError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_out(&r)?;

	if let Err(ref errors) = form.validate_args((s.db(), &r)) {
		return Ok(super::response::render_form_errors(
			&r,
			"register.html",
			errors,
			None,
		)?);
	}
	let private_key = if let Some(k) = form.get_private_key() {
		k
	} else {
		openssl::rsa::Rsa::generate(crate::consts::RSA_KEY_SIZE)?
	};
	s.db().create_user(&form.username, &form.password, &private_key).await?;
	super::flash::add(&r, "You have registered.", "success")?;
	Ok(super::response::redirect_static(&r, "login_get")?)
}

#[actix_web::get("/emails/send/")]
pub(crate) async fn send_email_get(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, SendEmailGetError> {
	super::auth::validate_logged_in(&r)?;
	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();

	let friends_ = s
		.db()
		.get_friends(&user)
		.await
		.map_err(SendEmailGetError::GetFriends)?;
	let nodes =
		s.db().get_nodes(&user).await.map_err(SendEmailGetError::GetNodes)?;
	validate_at_least_one_friend_and_one_node!(
		&r,
		friends_,
		nodes,
		SendEmailGetError,
	);

	let context = context! {"friends" => &friends_};
	Ok(super::response::render(
		&r,
		"send-email.html",
		Some(context),
		actix_web::http::StatusCode::OK,
		true,
	)?)
}

#[actix_web::post("/emails/send/")]
pub(crate) async fn send_email_post(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	multipart: actix_multipart::Multipart,
) -> Result<actix_web::HttpResponse, SendEmailPostError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_in(&r)?;
	// We can use `Option::unwrap` because of `super::auth::validate_logged_in`
	let user = super::auth::get_current_user(&r)?.unwrap();

	let friends_ = s
		.db()
		.get_friends(&user)
		.await
		.map_err(SendEmailPostError::GetFriends)?;
	let nodes =
		s.db().get_nodes(&user).await.map_err(SendEmailPostError::GetNodes)?;
	validate_at_least_one_friend_and_one_node!(
		&r,
		friends_,
		nodes,
		SendEmailPostError,
	);
	let context = context! {"friends" => &friends_};

	// Make and validate the form
	let form: super::forms::Email =
		super::multipart::extract(multipart).await?;
	if let Err(errors) = form.validate_args(&r) {
		return Ok(super::response::render_form_errors(
			&r,
			"send-email.html",
			&errors,
			Some(context),
		)?);
	}

	// Get public and private keys
	let recipient_public_key = form.get_recipient_public_key();
	let private_key = s
		.db()
		.get_user_private_key(&user)
		.await
		.map_err(SendEmailPostError::GetUserPrivateKey)?;

	// Make new email and get it's size
	let user_username = user.username().to_owned();
	let email_bytes = actix_web::web::block(move || {
		// Make and serialize an encrypted email package
		let d = form.into_email_data(user_username);
		let mut e = common::email::Email::new(&recipient_public_key, d)?;
		e.generate_proof_of_work();
		e.sign(&private_key)?;
		let rv = bincode::serialize(&e)?;
		Ok::<_, SendEmailPostError>(rv)
	})
	.await??;

	// Make package with email bytes and validate it's size
	let package = common::package::Package::new(
		None,
		common::package::Action::SendEmail,
		email_bytes,
	);
	if package.is_too_big()? {
		super::flash::add(&r, "Your email is too big.", "danger")
			.map_err(SendEmailPostError::EmailIsTooBigFlash)?;
		return Ok(super::response::render(
			&r,
			"send-email.html",
			Some(context),
			actix_web::http::StatusCode::OK,
			true,
		)?);
	}

	// Send the package with the encrypted email to each node and flash the
	// message
	let nodes_len = nodes.len();
	match common::helpers::send_email_to_nodes(
		package,
		nodes,
		nodes_len,
		s.config().proxy(),
	)
	.await?
	{
		0 => super::flash::add(&r, "Your email was not sent.", "danger")
			.map_err(SendEmailPostError::NotSentFlash)?,
		_ => super::flash::add(&r, "Your email was sent.", "success")
			.map_err(SendEmailPostError::SentFlash)?,
	}

	Ok(super::response::redirect_static(&r, "emails")?)
}

#[actix_web::post("/switch-f2f/")]
pub(crate) async fn switch_f2f(
	s: actix_web::web::Data<crate::state::State>,
	r: actix_web::HttpRequest,
	form: actix_web::web::Form<super::forms::Csrf>,
) -> Result<actix_web::HttpResponse, SwitchF2fError> {
	use validator::ValidateArgs as _;
	super::auth::validate_logged_in(&r)?;

	match form.validate_args(&r) {
		Ok(()) => {
			// We can use `Option::unwrap` because of
			// `super::auth::validate_logged_in`
			let user = super::auth::get_current_user(&r)?.unwrap();
			if s.db().switch_user_f2f(&user).await? {
				super::flash::add(&r, "You have enabled F2F mode.", "success")
					.map_err(SwitchF2fError::EnabledFlashError)?;
			} else {
				super::flash::add(&r, "You have disabled F2F mode.", "danger")
					.map_err(SwitchF2fError::DisabledFlashError)?;
			}
		}
		Err(ref errors) => super::flash::add_form_errors(&r, errors)?,
	}
	Ok(super::response::redirect_static(&r, "profile")?)
}
