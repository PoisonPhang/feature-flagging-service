//!
//! Data model for Users
//! 

use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

///
/// # User
/// Data object for users
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// ObjectID generated my MongoDB
    #[serde(alias = "_id")]
    pub id: ObjectId, 
    /// User Name
    name: String,
    /// User email
    email: String,
    /// User password hash
    password_hash: String,
}

impl Default for User {
    fn default() -> User {
        User {
            id: ObjectId::default(),
            name: "default_user".to_string(),
            email: "default_user_email".to_string(),
            password_hash: "default_password_hash".to_string(),
        }
    }
}
