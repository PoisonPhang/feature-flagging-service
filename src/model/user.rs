//!
//! Data model for Users
//! 

use serde::{Deserialize, Serialize};

///
/// # User
/// Data object for users
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// User Name
    name: String,
    /// User email
    email: String,
    /// User password hash
    password_hash: String,
}

impl User {
    ///
    /// # User::new()
    /// Creates and returns a new `User` with the provided fields
    /// 
    /// ## Example
    /// ```
    /// let example_user = User::new("example_user".to_string(), "user@example.org".to_string(), "example_hash".to_string());
    /// ```
    /// 
    pub fn new(name: String, email: String, password_hash: String) -> User {
        User {
            name,
            email,
            password_hash,
        }
    }
}
