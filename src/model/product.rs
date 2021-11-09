//! Data model for Products

use std::vec::Vec;

use serde::{Deserialize, Serialize};

/// Data object for products
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
  /// String generated my MongoDB
  #[serde(alias = "_id", skip_serializing)]
  pub id: String,
  /// Product Name
  pub name: String,
  /// List of controlled features in the product
  pub features: Vec<String>,
  /// List of product user ids
  pub users: Vec<String>,
}

impl Default for Product {
  fn default() -> Product {
    Product {
      id: "default_id".to_string(),
      name: "default_product".to_string(),
      features: Vec::new(),
      users: Vec::new(),
    }
  }
}

impl Product {
  pub fn builder() -> ProductBuilder {
    ProductBuilder::new()
  }
}

#[derive(Clone)]
pub struct ProductBuilder {
  /// String unique ID
  pub id: String,
  /// Product Name
  pub name: String,
  /// List of controlled features in the product by ID
  pub features: Vec<String>,
  /// List of product user IDs
  pub users: Vec<String>,
}

impl Default for ProductBuilder {
  fn default() -> ProductBuilder {
    let default_product = Product::default();
    ProductBuilder {
      id: default_product.id,
      name: default_product.name,
      features: default_product.features,
      users: default_product.users,
    }
  }
}

impl ProductBuilder {
  fn new() -> ProductBuilder {
    ProductBuilder::default()
  }

  pub fn with_id(mut self, id: String) -> ProductBuilder {
    self.id = id;
    self
  }

  pub fn with_name(mut self, name: &str) -> ProductBuilder {
    self.name = name.to_string();
    self
  }

  pub fn with_features(mut self, features: Vec<String>) -> ProductBuilder {
    self.features = features;
    self
  }

  pub fn with_users(mut self, users: Vec<String>) -> ProductBuilder {
    self.users = users;
    self
  }

  pub fn build(self) -> Product {
    Product {
      id: self.id,
      name: self.name,
      features: self.features,
      users: self.users,
    }
  }
}
