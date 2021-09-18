//!
//! Data model for Products
//! 

use std::collections::HashMap;
use std::vec::Vec;

use bson::oid::ObjectId;

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
    pub fn new(name: String, features: Vec<ObjectId>) -> Product {
        Product {
            name,
            features,
        }
    }
}
