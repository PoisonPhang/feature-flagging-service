//! User authentication utilities

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;

/// Contains a hash map of user tokens to validate that a user is logged in
pub struct AuthTokens {
    /// `HashMap` relating a list of tokens to a user ID
    user_tokens: HashMap<ObjectId, Vec<String>>,
  }
  
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
      let token = "token".to_string();
      
      let tokens_new = match self.user_tokens.get(&user_id) {
        Some(tokens_old) => {
          tokens_old.to_owned().push(token.to_owned());
        }
        None => {
          vec!(token)
        }
      };
  
      self.user_tokens.insert(user_id, tokens_new);
  
      token
    }
  
    /// Checks if a token is authenticated under a specific user
    pub fn check_for(&self, user_id_hex: &str, token: String) -> bool {
      let user_id = match ObjectId::parse_str(user_id_hex) {
        Ok(value) => value,
        Err(e) => {
          print!("Error parsing user_id (ObjectId) from user_id_hex: {:?}", e);
          return false
        }
      };
  
      match self.user_tokens.get(&user_id) {
        Some(tokens) => {
          tokens.contains(&token)
        }
        None => false
      }
    }
  }