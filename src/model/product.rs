//!
//! Data model for Products
//! 

use std::collections::HashMap;

use crate::model::flag;

///
/// # Product
/// Data object for products
/// 
pub struct Product {
    /// Product Name
    pub name: String,
    /// List of controlled features in the product
    pub features: HashMap<usize, flag::FeatureFlag>
}