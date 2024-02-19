/// For `service` `error`s to render traceback with diffirent `status`es.
///
/// `_ => INTERNAL_SERVER_ERROR` statement included for all `error`s by
/// default.
macro_rules! impl_error {
	(
		$error:ident $(:)?
		$(
			$($pattern:pat_param)|+ $(if $guard:expr)? => $status:ident
		)*
	) => {
		impl std::fmt::Debug for $error {
			#[inline]
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				fmt_traceback(self, f)
			}
		}

		impl actix_web::ResponseError for $error {
			#[must_use]
			fn status_code(&self) -> actix_web::http::StatusCode {
				match self {
					$(
						$($pattern)|+ $(if $guard)? => actix_web::http::StatusCode::$status,
					)*
					_ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
				}
			}

			#[must_use]
			fn error_response(&self) -> actix_web::HttpResponse {
				actix_web::HttpResponse::build(self.status_code())
					.body(format!("{:?}", self))
			}
		}
	}
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum AddFriendGetError {
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum AddFriendPostError {
	#[error("Failed to add a friend.")]
	AddFriend(#[from] anyhow::Error),
	#[error("Failed to make a flash.")]
	Flash(#[from] AddFlashError),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to render form errors.")]
	RenderFormErrors(#[from] RenderFormErrorsError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum AddNodeGetError {
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum AddNodePostError {
	#[error("Failed to add a node.")]
	AddNode(#[from] anyhow::Error),
	#[error("Failed to make a flash.")]
	Flash(#[from] AddFlashError),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to render form errors.")]
	RenderFormErrors(#[from] RenderFormErrorsError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum CheckCsrfTokenError {
	#[error("There is an invalid token in cookie.")]
	Invalid,
	#[error("CSRF cookie not found.")]
	NotFound,
	#[error("Failed to parse cookies.")]
	Parse(#[from] actix_web::cookie::ParseError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ConvertPemBase64ToPrivateKeyError {
	#[error("Failed to decode base64.")]
	Base64(#[from] base64::DecodeError),
	#[error("Failed to build a private key.")]
	Build(#[from] openssl::error::ErrorStack),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ConvertPemBase64ToPublicKeyError {
	#[error("Failed to decode base64.")]
	Base64(#[from] base64::DecodeError),
	#[error("Failed to build a public key.")]
	Build(#[from] openssl::error::ErrorStack),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum DeleteAccountGetError {
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged in.")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum DeleteAccountPostError {
	#[error("Failed to delete a user.")]
	Delete(#[from] anyhow::Error),
	#[error("Failed to flash.")]
	Flash(#[from] AddFlashError),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to render a form errors.")]
	RenderFormErrors(#[from] RenderFormErrorsError),
	#[error("Failed to validate that user is logged in.")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum DeleteFriendError {
	#[error("Failed to delete a friend.")]
	Delete(#[from] anyhow::Error),
	#[error("Failed to make a flash.")]
	Flash(#[from] AddFlashError),
	#[error("Failed to flash form errors.")]
	FlashFormErrors(#[from] AddFormErrorFlashesError),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum DeleteNodeError {
	#[error("Failed to delete a node.")]
	Delete(#[from] anyhow::Error),
	#[error("Failed to make a flash.")]
	Flash(#[from] AddFlashError),
	#[error("Failed to flash form errors.")]
	FlashFormErrors(#[from] AddFormErrorFlashesError),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum EmailError {
	#[error("Failed to check that friend exists by username.")]
	CheckFriendExistsByUsername(#[source] anyhow::Error),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get emails.")]
	GetEmail(#[source] anyhow::Error),
	#[error("Failed to get a friend.")]
	GetFriend(#[source] anyhow::Error),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum EmailsError {
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get emails.")]
	GetEmails(#[from] anyhow::Error),
	#[error("Invalid page.")]
	InvalidPage,
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ExtractFieldValueError {
	#[error("Failed to unwrap a chunk.")]
	UnwrapChunk(#[from] actix_multipart::MultipartError),
	#[error("Failed to convert a chunk to UTF-8.")]
	Utf8(#[from] std::str::Utf8Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ExtractFileFieldValueError {
	#[error("Failed to unwrap a chunk.")]
	UnwrapChunk(#[from] actix_multipart::MultipartError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ExtractMultipartError {
	#[error("Failed to convert fields to a return type.")]
	ReturnTypeFromFields(#[from] serde_json::Error),
	#[error("Failed to extract field value.")]
	ExtractFieldValue(#[from] ExtractFieldValueError),
	#[error("Failed to extract file field value.")]
	ExtractFileFieldValue(#[from] ExtractFileFieldValueError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum AddFlashError {
	#[error("Failed to get flashes.")]
	Get(#[from] actix_session::SessionGetError),
	#[error("Failed to insert flashes.")]
	Insert(#[from] actix_session::SessionInsertError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum AddFormErrorFlashesError {
	#[error("Failed to flash.")]
	Flash(#[from] AddFlashError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum FriendsError {
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get friends.")]
	GetFriends(#[from] anyhow::Error),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum GenerateCsrfTokenError {
	#[error("Failed to generate a token.")]
	Generate(#[from] common::error::GenerateRandomBytesError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum GetCurrentUserError {
	#[error("Failed to get an id.")]
	GetId(#[from] anyhow::Error),
	#[error("Failed to convert str to a user.")]
	UserFromJson(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum GetNodeEmailsCountError {
	#[error("Failed to convert public key to PEM.")]
	PublicKeyToPem(#[from] openssl::error::ErrorStack),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum IndexError {
	#[error("Failed to render.")]
	Render(#[from] RenderError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum LoadEmailsError {
	#[error("Failed to flash.")]
	Flash(#[from] AddFlashError),
	#[error("Failed to flash form errors.")]
	FlashFormErrors(#[from] AddFormErrorFlashesError),
	#[error("Failed to get nodes.")]
	GetNodes(#[source] anyhow::Error),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get a user private key.")]
	GetUserPrivateKey(#[source] anyhow::Error),
	#[error("Failed to join a task.")]
	Join(#[from] tokio::task::JoinError),
	#[error("Failed to load emails from node.")]
	LoadEmails(#[from] LoadNodeEmailsError),
	#[error("Failed to make a success static redirect.")]
	SuccessRedirectStatic(#[source] RedirectStaticError),
	#[error("Failed to make a validation static redirect.")]
	ValidationRedirectStatic(#[source] RedirectStaticError),
	#[error("Failed to validate that user is logged in.")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum LoadNodeEmailsError {
	#[error("Failed to check that email exists.")]
	CheckEmailExists(#[source] anyhow::Error),
	#[error("Failed to check that friend exists by public key.")]
	CheckFriendExistsByPublicKey(#[source] anyhow::Error),
	#[error("Failed to check user f2f status.")]
	CheckUserF2f(#[source] anyhow::Error),
	#[error("Failed to get emails count from node.")]
	GetNodeEmailsCount(#[from] GetNodeEmailsCountError),
	#[error("Failed to convert public key to PEM.")]
	PublicKeyToPem(#[from] openssl::error::ErrorStack),
	#[error("Failed to convert a request to bytes.")]
	RequestToBytes(#[from] bincode::Error),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum LoginGetError {
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate logged out.")]
	ValidateLoggedOut(#[from] ValidateLoggedOutError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum LoginPostError {
	#[error("Failed to flash the message.")]
	Flash(#[from] AddFlashError),
	#[error("Faied to get a user.")]
	GetUser(#[from] anyhow::Error),
	#[error("Failed to login a user.")]
	LoginUser(#[from] LoginUserError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to render a form errors.")]
	RenderFormErrors(#[from] RenderFormErrorsError),
	#[error("Failed to validate logged out.")]
	ValidateLoggedOut(#[from] ValidateLoggedOutError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum LogoutError {
	#[error("Failed to make a flash.")]
	Flash(#[from] AddFlashError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate logged in.")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum LoginUserError {
	#[error("Failed to login.")]
	Login(#[from] anyhow::Error),
	#[error("Failed to convert user to a JSON.")]
	UserToJson(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum MakeTeraBaseContextError {
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to pop all flashes.")]
	PopAllFlashes(#[from] PopAllFlashesError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum NodesGetError {
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get nodes.")]
	GetNodes(#[from] anyhow::Error),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum NodesPostError {
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get nodes.")]
	GetNodes(#[from] anyhow::Error),
	#[error("Failed to join a task.")]
	Join(#[from] tokio::task::JoinError),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum PopAllFlashesError {
	#[error("Failed to get flashes.")]
	Get(#[from] actix_session::SessionGetError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ProfileError {
	#[error("Failed to check user's F2F.")]
	CheckUserF2f(#[source] anyhow::Error),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get user's private key.")]
	GetUserPrivateKey(#[from] anyhow::Error),
	#[error("Failed to convert a private key to public key.")]
	PublicKeyToPem(#[from] openssl::error::ErrorStack),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged out..")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum RedirectError {
	#[error("Failed to generate a url.")]
	GenerateUrl(#[from] actix_web::error::UrlGenerationError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum RedirectStaticError {
	#[error("Failed to generate a url.")]
	Redirect(#[from] RedirectError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum RegisterGetError {
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate logged out.")]
	ValidateLoggedOut(#[from] ValidateLoggedOutError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum RegisterPostError {
	#[error("Failed to create a user.")]
	Create(#[from] anyhow::Error),
	#[error("Failed to make a flash.")]
	Flash(#[from] AddFlashError),
	#[error("Failed to generate a private key.")]
	GeneratePrivateKey(#[from] openssl::error::ErrorStack),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to render form's errors.")]
	RenderFormErrors(#[from] RenderFormErrorsError),
	#[error("Failed to validate logged out.")]
	ValidateLoggedOut(#[from] ValidateLoggedOutError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum RenderError {
	#[error("Failed to render using tera.")]
	Render(#[from] tera::Error),
	#[error("Failed to generate csrf token.")]
	GenerateCsrfToken(#[from] GenerateCsrfTokenError),
	#[error("Failed to make base context.")]
	MakeBaseContext(#[from] MakeTeraBaseContextError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum RenderFormErrorsError {
	#[error("Failed to render.")]
	Render(#[from] RenderError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum SendEmailGetError {
	#[error("Failed to make a friend addition flash.")]
	AddFriendFlash(#[source] AddFlashError),
	#[error("Failed to make a static redirect to friend addition.")]
	AddFriendRedirectStatic(#[source] RedirectStaticError),
	#[error("Failed to make a node addition flash.")]
	AddNodeFlash(#[source] AddFlashError),
	#[error("Failed to make a static redirect to node addition.")]
	AddNodeRedirectStatic(#[source] RedirectStaticError),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get friends.")]
	GetFriends(#[source] anyhow::Error),
	#[error("Failed to get nodes.")]
	GetNodes(#[source] anyhow::Error),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to validate that user is logged in.")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum SendEmailPostError {
	#[error("Failed to make a friend addition flash.")]
	AddFriendFlash(#[source] AddFlashError),
	#[error("Failed to make a static redirect to friend addition.")]
	AddFriendRedirectStatic(#[source] RedirectStaticError),
	#[error("Failed to make a node addition flash.")]
	AddNodeFlash(#[source] AddFlashError),
	#[error("Failed to make a static redirect to node addition.")]
	AddNodeRedirectStatic(#[source] RedirectStaticError),
	#[error("Failed to block.")]
	Block(#[from] actix_web::error::BlockingError),
	#[error("Failed to check that email is too big.")]
	EmailIsTooBig(#[from] common::error::PackageIsTooBigError),
	#[error("Failed to make a flash that email is too big.")]
	EmailIsTooBigFlash(#[source] AddFlashError),
	#[error("Failed to convert an email to bytes.")]
	EmailToBytes(#[from] bincode::Error),
	#[error("Failed to extract multipart.")]
	ExtractMultipart(#[from] ExtractMultipartError),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to get friends.")]
	GetFriends(#[source] anyhow::Error),
	#[error("Failed to get nodes.")]
	GetNodes(#[source] anyhow::Error),
	#[error("Failed to get a user private key.")]
	GetUserPrivateKey(#[source] anyhow::Error),
	#[error("Failed to create a new email.")]
	NewEmail(#[from] common::error::NewEmailError),
	#[error("Failed to make a not sent flash.")]
	NotSentFlash(#[source] AddFlashError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to render.")]
	Render(#[from] RenderError),
	#[error("Failed to render a form errors.")]
	RenderFormErrors(#[from] RenderFormErrorsError),
	#[error("Failed to send an email to nodes.")]
	SendEmailToNodes(#[from] common::error::SendEmailToNodesError),
	#[error("Failed to make a sent flash.")]
	SentFlash(#[source] AddFlashError),
	#[error("Failed to sign an email.")]
	SignEmail(#[from] common::error::SignEmailError),
	#[error("Failed to validate that user is logged in.")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(thiserror::Error)]
#[non_exhaustive]
pub(crate) enum SwitchF2fError {
	#[error("Failed to flash the disabled message.")]
	DisabledFlashError(#[source] AddFlashError),
	#[error("Failed to flash the enabled message.")]
	EnabledFlashError(#[source] AddFlashError),
	#[error("Failed to flash form errors.")]
	FlashFormErrors(#[from] AddFormErrorFlashesError),
	#[error("Failed to get a current user.")]
	GetCurrentUser(#[from] GetCurrentUserError),
	#[error("Failed to make a static redirect.")]
	RedirectStatic(#[from] RedirectStaticError),
	#[error("Failed to switch user's F2F.")]
	SwitchUserF2f(#[from] anyhow::Error),
	#[error("Failed to validate that user is logged in.")]
	ValidateLoggedIn(#[from] ValidateLoggedInError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ValidateLoggedInError {
	#[error("User is logged out.")]
	LoggedOut,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub(crate) enum ValidateLoggedOutError {
	#[error("User is logged in.")]
	LoggedIn,
}

impl_error!(
	AddFriendGetError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	AddFriendPostError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	AddNodeGetError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	AddNodePostError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	CheckCsrfTokenError:
	Self::Invalid | Self::NotFound => BAD_REQUEST
);
impl_error!(
	DeleteAccountGetError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	DeleteAccountPostError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	DeleteFriendError:
	Self::Delete(e) if check_diesel_not_found_down(e) => NOT_FOUND
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	DeleteNodeError:
	Self::Delete(e) if check_diesel_not_found_down(e) => NOT_FOUND
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	EmailError:
	Self::GetEmail(e) if check_diesel_not_found_down(e) => NOT_FOUND
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	EmailsError:
	Self::InvalidPage => NOT_FOUND
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	FriendsError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	GetCurrentUserError:
	Self::GetId(_) => UNAUTHORIZED
);
impl_error!(IndexError);
impl_error!(
	LoadEmailsError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	LoginGetError:
	Self::ValidateLoggedOut(ValidateLoggedOutError::LoggedIn) => FORBIDDEN
);
impl_error!(
	LoginPostError:
	Self::ValidateLoggedOut(ValidateLoggedOutError::LoggedIn) => FORBIDDEN
);
impl_error!(
	LogoutError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	NodesGetError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	NodesPostError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	ProfileError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	RegisterGetError:
	Self::ValidateLoggedOut(ValidateLoggedOutError::LoggedIn) => FORBIDDEN
);
impl_error!(
	RegisterPostError:
	Self::ValidateLoggedOut(ValidateLoggedOutError::LoggedIn) => FORBIDDEN
);
impl_error!(
	SendEmailGetError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	SendEmailPostError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);
impl_error!(
	SwitchF2fError:
	Self::ValidateLoggedIn(ValidateLoggedInError::LoggedOut) => UNAUTHORIZED
);

#[inline]
#[must_use]
pub(super) fn check_diesel_not_found_down(e: &anyhow::Error) -> bool {
	e.downcast_ref::<diesel::result::Error>() == Some(&diesel::NotFound)
}

fn fmt_traceback(
	e: &(dyn std::error::Error + 'static),
	f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
	writeln!(f, "Error:")?;
	for source in e.sources() {
		writeln!(f, "\tCaused by: {source}")?;
	}
	Ok(())
}
