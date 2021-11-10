//! Data model for Users

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// Data object for users
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  /// String generated my MongoDB
  #[serde(alias = "_id", skip_serializing_if = "Option::is_none")]
  pub oid: Option<ObjectId>,
  /// User Name
  pub name: String,
  /// User email
  pub email: String,
  /// User password hash
  pub password_hash: String,
}

impl Default for User {
  fn default() -> User {
    User {
      oid: Default::default(),
      name: "default_user".to_string(),
      email: "default_user_email".to_string(),
      password_hash: "default_password_hash".to_string(),
    }
  }
}

impl User {
  pub fn builder() -> UserBuilder {
    UserBuilder::new()
  }
}

#[derive(Clone)]
pub struct UserBuilder {
  /// String generated my MongoDB
  oid: Option<ObjectId>,
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
      oid: default_user.oid,
      name: default_user.name,
      email: default_user.email,
      password_hash: default_user.password_hash,
    }
  }
}

impl UserBuilder {
  /// Creates a new `UserBuilder` with default fields from `UserBuilder::default()`
  fn new() -> UserBuilder {
    UserBuilder::default()
  }

  pub fn with_oid(mut self, oid: ObjectId) -> UserBuilder {
    self.oid = Some(oid);
    self
  }

  pub fn with_name(mut self, name: &str) -> UserBuilder {
    self.name = name.to_string();
    self
  }

  pub fn with_email(mut self, email: &str) -> UserBuilder {
    self.email = email.to_string();
    self
  }

  pub fn with_password_hash(mut self, password_hash: &str) -> UserBuilder {
    self.password_hash = password_hash.to_string();
    self
  }

  /// Builds itself into and returns a `User` consuming the `UserBuilder`
  ///
  /// # Examples
  /// Basic usage:
  /// ```
  /// let user_builder = User::builder();
  /// let user = user_builder.with_name("examples_name").build();
  /// ```
  /// To avoid consumption, use `clone()`
  /// ```
  /// let user_builder = User::builder();
  /// let user_one = user_builder.with_id("examples_name_one").clone().build();
  /// let user_two = user_builder.with_id("examples_name_two").clone().build();
  /// ```
  pub fn build(self) -> User {
    User {
      oid: self.oid,
      name: self.name,
      email: self.email,
      password_hash: self.password_hash,
    }
  }
}
