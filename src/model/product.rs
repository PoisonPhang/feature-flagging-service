//! Data model for Products

use mongodb::bson::oid::ObjectId;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

/// Data object for products
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
  /// String generated my MongoDB
  #[serde(alias = "_id", skip_serializing_if = "Option::is_none")]
  pub oid: Option<ObjectId>,
  /// Product Name
  pub name: String,
  /// List of product user ids
  pub users: Vec<String>,
}

impl Default for Product {
  fn default() -> Product {
    Product {
      oid: Default::default(),
      name: "default_product".to_string(),
      users: Vec::new(),
    }
  }
}

impl Product {
  pub fn builder() -> ProductBuilder {
    ProductBuilder::new()
  }

  pub fn get_spec_safe_product(&self) -> SpecSafeProduct {
    SpecSafeProduct {
      oid: match self.oid {
        Some(oid) => oid.to_hex(),
        None => ObjectId::default().to_hex(),
      },
      name: self.name,
      users: self.users,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SpecSafeProduct {
  pub oid: String,
  /// Product Name
  pub name: String,
  /// List of product user ids
  pub users: Vec<String>,
}

#[derive(Clone)]
pub struct ProductBuilder {
  /// String unique ID
  pub oid: Option<ObjectId>,
  /// Product Name
  pub name: String,
  /// List of product user IDs
  pub users: Vec<String>,
}

impl Default for ProductBuilder {
  fn default() -> ProductBuilder {
    let default_product = Product::default();
    ProductBuilder {
      oid: default_product.oid,
      name: default_product.name,
      users: default_product.users,
    }
  }
}

impl ProductBuilder {
  fn new() -> ProductBuilder {
    ProductBuilder::default()
  }

  pub fn with_oid(mut self, oid: ObjectId) -> ProductBuilder {
    self.oid = Some(oid);
    self
  }

  pub fn with_name(mut self, name: &str) -> ProductBuilder {
    self.name = name.to_string();
    self
  }

  pub fn with_users(mut self, users: Vec<String>) -> ProductBuilder {
    self.users = users;
    self
  }

  pub fn build(self) -> Product {
    Product {
      oid: self.oid,
      name: self.name,
      users: self.users,
    }
  }
}
