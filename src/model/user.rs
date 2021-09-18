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
    pub fn new(name: String, email: String, password_hash: String) -> User {
        User {
            name,
            email,
            password_hash,
        }
    }
}
