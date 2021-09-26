//!
//! Data model for Products
//! 

use std::collections::HashMap;
use std::vec::Vec;

use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

use crate::model::flag;

///
/// # Product
/// Data object for products
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    /// Product Name
    pub name: String,
    /// List of controlled features in the product
    pub features: Vec<ObjectId>,
}

impl Product {
    ///
    /// # Product::new()
    /// Creates and returns a new `Product` with the provided fields
    /// 
    /// Declare as mutable if you want to alter the vector of features
    /// 
    /// ## Example
    /// ```
    /// let mut product = Product::new("example_product".to_string(), vec!());
    /// ```
    pub fn new(name: String, features: Vec<ObjectId>) -> Product {
        Product {
            name,
            features,
        }
    }
}
