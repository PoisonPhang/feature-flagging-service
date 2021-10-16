//!
//! Data model for Products
//! 

use std::vec::Vec;

use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

///
/// Data object for products
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    /// ObjectID generated my MongoDB
    #[serde(alias = "_id", skip_serializing)]
    pub id: ObjectId, 
    /// Product Name
    pub name: String,
    /// List of controlled features in the product
    pub features: Vec<ObjectId>,
}

impl Default for Product {
    fn default() -> Product {
        Product {
            id: ObjectId::default(),
            name: "default_product".to_string(),
            features: Vec::new(),
        }
    }
}
