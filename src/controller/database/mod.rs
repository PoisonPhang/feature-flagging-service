//!
//! Database connection usage and management
//! 

use dotenv;
use crate::model::flag::FeatureFlag;

pub mod mongo;

enum ConnectionType {
    MongoDB,
}

pub struct ConnectionManager {
    connection_type: ConnectionType,
}

impl ConnectionManager {
    pub fn new() -> ConnectionManager {
        let connection_type = match dotenv::var("DATABASE_CONNECTION_TYPE") {
            Ok(value) => match value.as_str() {
                "mongodb" => ConnectionType::MongoDB,
                _ => panic!("Unrecoverable error. Unrecognized 'DATABASE_CONNECTION_TYPE': {}", value)
            },
            Err(e) => {
                panic!("Unrecoverable error. Error reading 'DATABASE_CONNECTION_TYPE' from '.env': {:?}", e)
            }
        };

        ConnectionManager {
            connection_type
        }
    }

    pub async fn get_feature_flag(&self, product: &str, flag_name: &str) -> Option<FeatureFlag> {
        match &self.connection_type {
            ConnectionType::MongoDB => {
                match mongo::mongo::get_feature_flag(product, flag_name).await {
                    Ok(value) => return Some(value),
                    Err(e) => {
                        print!("Error getting feature `{}`. Returning Option::None. Error: {:?}", flag_name, e);
                        return None
                    }
                }
            }
        }
    }
}
