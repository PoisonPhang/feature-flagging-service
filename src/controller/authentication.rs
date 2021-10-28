//! User authentication utilities

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

const USER_ID: &str = "user_id";
const AUTH_TOKEN: &str = "auth_token";

#[derive(Debug)]
pub enum UserAuthError {
  NoUserId,
  NoAuthToken,
  Invalid,
}

pub struct UserAuth;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuth {
  type Error = UserAuthError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    // Get user id from cookie
    let user_id = match request.cookies().get_private(USER_ID) {
      Some(value) => value.value().to_owned(), // Get value found from cookies
      None => return Outcome::Failure((Status::BadRequest, UserAuthError::NoUserId))
    };
    // Get auth token from cookie
    let auth_token = match request.cookies().get_private(AUTH_TOKEN) {
      Some(value) => value.value().to_owned(), // Get value found from cookies
      None => return Outcome::Failure((Status::BadRequest, UserAuthError::NoAuthToken))
    };
    // Get current auth tokens from state
    let tokens_mut = match request.rocket().state::<Arc<Mutex<AuthTokens>>>() {
      Some(value) => value,
      None => return Outcome::Failure((Status::BadRequest, UserAuthError::Invalid))
    };
    // Lock current tokens for reading
    let tokens = match tokens_mut.lock() {
      Ok(value) => value,
      Err(poisoned) => poisoned.into_inner(), // recover from poisoned mutex
    };

    if tokens.check_for(&user_id, &auth_token) {
      return Outcome::Success(Self)
    }

    Outcome::Failure((Status::BadRequest, UserAuthError::Invalid))
  }
}

/// Contains a hash map of user tokens to validate that a user is logged in
pub struct AuthTokens {
    /// `HashMap` relating a list of tokens to a user ID
    user_tokens: HashMap<ObjectId, Vec<String>>,
  }

  // TODO implement FromRequest https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html
  
  impl AuthTokens {
    /// Creates and returns a new `AuthTokens` struct
    pub fn new() -> AuthTokens {
      AuthTokens {
        user_tokens: HashMap::new()
      }
    }
  
    /// Creates a new token for the specified user, adds it to the user tokens `HashMap` and returns the token
    pub fn add_token(&mut self, user_id: ObjectId) -> String {
      // TODO generate real token
      let token = "token";
      
      let tokens_new = match self.user_tokens.get(&user_id) {
        Some(tokens_old) => {
          tokens_old.to_owned().push(token.to_owned());
          tokens_old.to_owned()
        }
        None => {
          vec!(token.to_string())
        }
      };
  
      self.user_tokens.insert(user_id, tokens_new);
  
      token.to_string()
    }
  
    /// Checks if a token is authenticated under a specific user
    pub fn check_for(&self, user_id_hex: &str, token: &str) -> bool {
      let user_id = match ObjectId::parse_str(user_id_hex) {
        Ok(value) => value,
        Err(e) => {
          print!("Error parsing user_id (ObjectId) from user_id_hex: {:?}", e);
          return false
        }
      };
  
      match self.user_tokens.get(&user_id) {
        Some(tokens) => {
          tokens.contains(&token.to_owned())
        }
        None => false
      }
    }
  }