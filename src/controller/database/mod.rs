//!
//! Database connection usage and management
//! 

pub mod mongo;

use crate::model::flag::FeatureFlag;

enum ConnectionType {
    MongoDB,
}

struct ConnectionManager {
    connection_type: ConnectionType,
}

impl ConnectionManager {
    pub async fn get_feature_flag(&self, product: &str, flag_name: &str) -> Option<FeatureFlag> {
        match &self.connection_type {
            ConnectionType::MongoDB => {
                match mongo::mongo::get_feature_flag(product, flag_name).await {
                    Ok(value) => return Some(value),
                    Err(e) => {
                        print!("Error getting feature `{}` returning Option::None. Error: {:?}", flag_name, e);
                        return None
                    }
                }
            }
        }
    }
}