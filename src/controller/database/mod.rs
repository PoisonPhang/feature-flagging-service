//! Database connection usage and management

use dotenv;

use mongodb::bson::oid::ObjectId;

use crate::model::flag::{FeatureFlag, FeatureFlagBuilder};
use crate::model::product::{Product, ProductBuilder};
use crate::model::user::{AccountType, User, UserBuilder};

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

  pub async fn get_products(&self, user_id: &str) -> Vec<Product> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::get_products(user_id).await {
        Ok(products) => return products,
        Err(e) => {
          println!(
            "Error getting products for user w/ ID: {}. Returning empty Vec. Error {:?}",
            user_id, e
          );
          return vec![];
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

  pub async fn get_feature_flags(&self, product_id: &str) -> Vec<FeatureFlag> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::get_feature_flags(product_id).await {
        Ok(feature_flags) => return feature_flags,
        Err(e) => {
          println!(
            "Error getting features for product_id '{}'. Returning empty Vec. Error: {:?}",
            product_id, e
          );
          return vec![];
        }
      },
    }
  }

  pub async fn update_feature_flag(&self, feature_flag_id: &str, updated: FeatureFlag) -> bool {
    match &self.connection_type {
      ConnectionType::MongoDB => {
        let id: ObjectId = match ObjectId::parse_str(feature_flag_id) {
          Ok(id) => id,
          Err(_) => return false,
        };

        match mongo::update_feature_flag(id, updated).await {
          Ok(_) => return true,
          Err(e) => {
            println!("Error updating feature flag. Error: {:?}", e);
            return false
          },
        };
      }
    }
  }

  /// Given a product id, and flag name, returns a fully constructed `User`
  ///
  /// Returns `User` inside of an `Option<User>`. If anything goes wrong, this function will return `None`
  pub async fn get_user(&self, user_email: Option<&str>, user_id: Option<&str>) -> Option<User> {
    if user_email.is_none() && user_id.is_none() {
      println!("Error getting user, must provide at least one `user_email` or `user_id`");
      return None;
    }

    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::get_user(user_email, user_id).await {
        Ok(user) => return user,
        Err(e) => {
          println!(
            "Error getting user from email '{}' and/or id '{}'. Returning Option::None. Error: {:?}",
            user_email.unwrap_or("[Not Provided]"),
            user_id.unwrap_or("[Not Provided]"),
            e
          );
          return None;
        }
      },
    }
  }

  pub async fn get_users(&self, account_type: Option<AccountType>) -> Vec<User> {
    match &self.connection_type {
      ConnectionType::MongoDB => match mongo::get_users(account_type).await {
        Ok(users) => return users,
        Err(e) => {
          println!("Error getting users. Returning empty list: Error: {:?}", e);
          return vec!()
        }
      }
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
