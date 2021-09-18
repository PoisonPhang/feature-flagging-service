//!
//! Data model for Products
//! 

use std::collections::HashMap;

use crate::model::flag;

pub struct Product {
    pub id: usize,
    pub name: String,
    pub features: HashMap<usize, flag::FeatureFlag>
}
