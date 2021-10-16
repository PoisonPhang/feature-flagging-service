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
    #[serde(alias = "_id", skip_serializing)]
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

impl User {
    fn builder() -> UserBuilder {
        UserBuilder::new()
    }
}

pub struct UserBuilder {
        /// ObjectID generated my MongoDB
        pub id: ObjectId, 
        /// User Name
        name: String,
        /// User email
        email: String,
        /// User password hash
        password_hash: String,
}

impl Default for UserBuilder {
    fn default() -> UserBuilder {
        let default_user = User::default();

        UserBuilder {
            id: default_user.id,
            name: default_user.name,
            email: default_user.email,
            password_hash: default_user.password_hash,
        }
    }
}

impl UserBuilder {
    fn new() -> UserBuilder {
        UserBuilder::default()
    }

    fn with_id(mut self, id: ObjectId) -> UserBuilder {
        self.id = id;
        self
    }

    fn with_name(mut self, name: String) -> UserBuilder {
        self.name = name;
        self
    }

    fn with_email(mut self, email: String) -> UserBuilder {
        self.email = email;
        self
    }

    fn with_password_hash(mut self, password_hash: String) -> UserBuilder {
        self.password_hash = password_hash;
        self
    }

    fn build(self) -> User {
        User {
            id: self.id,
            name: self.name,
            email: self.email,
            password_hash: self.password_hash,
        }
    }
}
