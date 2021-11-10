//! Database connection usage and management

use dotenv;

use crate::model::flag::{FeatureFlag, FeatureFlagBuilder};
use crate::model::product::{Product, ProductBuilder};
use crate::model::user::{User, UserBuilder};

pub mod mongo;

enum ConnectionType {
  MongoDB,
}

/// Manager for database connections
pub struct ConnectionManager {
  /// Type of the database driver
  connection_type: ConnectionType,
}

impl ConnectionManager {
  /// Constructs and returns a new `ConnectionManager`
  pub fn new() -> ConnectionManager {
    let connection_type = match dotenv::var("DATABASE_CONNECTION_TYPE") {
      Ok(value) => match value.as_str() {
        "mongodb" => ConnectionType::MongoDB,
        _ => panic!(
          "Unrecoverable error. Unrecognized 'DATABASE_CONNECTION_TYPE': {}",
          value
        ),
      },
      Err(e) => {
        panic!(
          "\nUnrecoverable error. Error reading 'DATABASE_CONNECTION_TYPE' from '.env': {:?}\n",
          e
        )
      }
    };

    ConnectionManager { connection_type }
  }

  /// Given a product name, returns a fully constructed `Product` from the database
  ///
  /// Returns `Product` inside of an `Option<Product>`. If anything goes wrong, this function will return `None`
  pub async fn get_product(&self, product_name: &str) -> Option<Product> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::get_product(product_name).await {
        Ok(product) => return product,
        Err(e) => {
          println!(
            "Error getting product '{}'. Returning Option::None. Error {:?}",
            product_name, e
          );
          return None;
        }
      },
    }
  }

  /// Given a product id, and flag name, returns a fully constructed `FeatureFlag`
  ///
  /// Returns `FeatureFlag` inside of an `Option<FeatureFlag>`. If anything goes wrong, this function will return `None`
  pub async fn get_feature_flag(&self, product_id: &str, flag_name: &str) -> Option<FeatureFlag> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::get_feature_flag(product_id, flag_name).await {
        Ok(feature_flag) => return feature_flag,
        Err(e) => {
          println!(
            "Error getting feature '{}'. Returning Option::None. Error: {:?}",
            flag_name, e
          );
          return None;
        }
      },
    }
  }

  /// Given a product id, and flag name, returns a fully constructed `User`
  ///
  /// Returns `User` inside of an `Option<User>`. If anything goes wrong, this function will return `None`
  pub async fn get_user(&self, user_email: &str) -> Option<User> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::get_user(user_email).await {
        Ok(user) => return user,
        Err(e) => {
          println!(
            "Error getting user from email '{}'. Returning Option::None. Error: {:?}",
            user_email, e
          );
          return None;
        }
      },
    }
  }

  pub async fn create_product(&self, product_builder: ProductBuilder) -> Option<Product> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::create_product(product_builder).await {
        Ok(value) => return Some(value),
        Err(e) => {
          println!("Error creating product. Returning Option::None. Error {:?}", e);
          return None;
        }
      },
    }
  }

  pub async fn create_flag(&self, flag_builder: FeatureFlagBuilder) -> Option<FeatureFlag> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::create_flag(flag_builder).await {
        Ok(value) => return Some(value),
        Err(e) => {
          println!("Error creating flag. Returning Option::None. Error {:?}", e);
          return None;
        }
      },
    }
  }

  /// Creates a user from a given `UserBuilder`
  ///
  /// It's expected that all values besides `UserBuilder.id` are set. `UserBuilder.id` will be set by the database
  pub async fn create_user(&self, user_builder: UserBuilder) -> Option<User> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::create_user(user_builder).await {
        Ok(value) => return Some(value),
        Err(e) => {
          println!("Error creating user. Returning Option::None. Error {:?}", e);
          return None;
        }
      },
    }
  }
}
