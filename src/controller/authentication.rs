//! User authentication utilities

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_okapi::okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket_okapi::{
  gen::OpenApiGenerator,
  request::{OpenApiFromRequest, RequestHeaderInput},
};

const USER_ID: &str = "user_id";
const AUTH_TOKEN: &str = "auth_token";

#[derive(Debug)]
pub enum UserAuthError {
  NoUserId,
  NoAuthToken,
  Invalid,
}

/// Custom rocket request guard for request where cookie based user authentication is required
pub struct UserAuth;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuth {
  type Error = UserAuthError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    // Get user id from cookie
    let user_id = match request.cookies().get_private(USER_ID) {
      Some(value) => value.value().to_owned(), // Get value found from cookies
      None => return Outcome::Failure((Status::BadRequest, UserAuthError::NoUserId)),
    };
    // Get auth token from cookie
    let auth_token = match request.cookies().get_private(AUTH_TOKEN) {
      Some(value) => value.value().to_owned(), // Get value found from cookies
      None => return Outcome::Failure((Status::BadRequest, UserAuthError::NoAuthToken)),
    };
    // Get current auth tokens from state
    let tokens_mut = match request.rocket().state::<Arc<Mutex<AuthTokens>>>() {
      Some(value) => value,
      None => return Outcome::Failure((Status::BadRequest, UserAuthError::Invalid)),
    };
    // Lock current tokens for reading
    let tokens = match tokens_mut.lock() {
      Ok(value) => value,
      Err(poisoned) => poisoned.into_inner(), // recover from poisoned mutex
    };

    if tokens.check_for(&user_id, &auth_token) {
      return Outcome::Success(Self);
    }

    Outcome::Failure((Status::BadRequest, UserAuthError::Invalid))
  }
}

impl<'a> OpenApiFromRequest<'a> for UserAuth {
  fn from_request_input(
    _gen: &mut OpenApiGenerator,
    _name: String,
    _required: bool,
  ) -> rocket_okapi::Result<RequestHeaderInput> {
    let security_scheme = SecurityScheme {
      description: Some("Requires an Bearer token to access, token is: `auth_token`.".to_owned()),
      // Setup data requirements.
      // In this case the header `Authorization: mytoken` needs to be set.
      data: SecuritySchemeData::Http {
        scheme: "bearer".to_owned(), // `basic`, `digest`, ...
        // Just gives use a hint to the format used
        bearer_format: Some("bearer".to_owned()),
      },
      extensions: Object::default(),
    };
    // Add the requirement for this route/endpoint
    // This can change between routes.
    let mut security_req = SecurityRequirement::new();
    // Each security requirement needs to be met before access is allowed.
    security_req.insert("UserAuth".to_owned(), Vec::new());
    // These vvvvvvv-----^^^^^^^^ values need to match exactly!
    Ok(RequestHeaderInput::Security(
      "UserAuth".to_owned(),
      security_scheme,
      security_req,
    ))
  }
}

/// Contains a hash map of user tokens to validate that a user is logged in
pub struct AuthTokens {
  /// `HashMap` relating a list of tokens to a user ID
  user_tokens: HashMap<String, Vec<String>>,
}

// TODO implement FromRequest https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html

impl AuthTokens {
  /// Creates and returns a new `AuthTokens` struct
  pub fn new() -> AuthTokens {
    AuthTokens {
      user_tokens: HashMap::new(),
    }
  }

  /// Creates a new token for the specified user, adds it to the user tokens `HashMap` and returns the token
  pub fn add_token(&mut self, user_id: &str) -> String {
    // TODO generate real token
    let token = "token";

    let tokens_new = match self.user_tokens.get(user_id) {
      Some(tokens_old) => {
        tokens_old.to_owned().push(token.to_owned());
        tokens_old.to_owned()
      }
      None => {
        vec![token.to_string()]
      }
    };

    self.user_tokens.insert(user_id.to_string(), tokens_new);

    token.to_string()
  }

  /// Removes a user token.
  ///
  /// Returns `false` if the the user was not found
  pub fn remove_token(&mut self, user_id: &str) -> bool {
      match self.user_tokens.remove(user_id) {
          Some(_) => true,
          None => false,
      }
  }

  /// Checks if a token is authenticated under a specific user
  pub fn check_for(&self, user_id: &str, token: &str) -> bool {
    match self.user_tokens.get(user_id) {
      Some(tokens) => tokens.contains(&token.to_owned()),
      None => false,
    }
  }
}
