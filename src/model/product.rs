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

impl Product {
    pub fn builder() -> ProductBuilder {
        ProductBuilder::new()
    }
}

#[derive(Clone)]
pub struct ProductBuilder {
    /// ObjectID generated my MongoDB
    pub id: ObjectId, 
    /// Product Name
    pub name: String,
    /// List of controlled features in the product
    pub features: Vec<ObjectId>,
}

impl Default for ProductBuilder {
    fn default() -> ProductBuilder {
        let default_product = Product::default();
        ProductBuilder {
            id: default_product.id,
            name: default_product.name,
            features: default_product.features,
        }
    }
}

impl ProductBuilder {
    fn new() -> ProductBuilder {
        ProductBuilder::default()
    }

    pub fn with_id(mut self, id: ObjectId) -> ProductBuilder {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: String) -> ProductBuilder {
        self.name = name;
        self
    }

    pub fn with_features(mut self, features: Vec<ObjectId>) -> ProductBuilder {
        self.features = features;
        self
    }

    pub fn build(self) -> Product {
        Product {
            id: self.id,
            name: self.name,
            features: self.features,
        }
    }
}
