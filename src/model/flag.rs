//!
//! Data model structures of the Feature Flag
//! 

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

///
/// Data Object for a Feature Flag
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlag {
    /// ObjectID generated my MongoDB
    #[serde(alias = "_id", skip_serializing)]
    pub id: ObjectId, 
    /// Flag Name
    pub name: String,
    /// Global enabled status of the flag (false trumps other statuses)
    pub enabled: bool,
    /// If client toggles are enabled
    pub client_toggle: bool,
    /// Type of release and relevant data
    pub release_type: ReleaseType,
}

impl FeatureFlag {
    pub fn evaluate(&self, user: Option<&str>) -> bool {

        // If no user is provided, only evaluate Global release type as potentially true
        let user = match user {
            Some(value) => value,
            None => {
                if self.enabled {
                    match &self.release_type {
                        ReleaseType::Global => {
                            return true
                        },
                        _ => {
                            return false
                        }
                    }
                }
                return false
            }
        };

        // Evaluate each release type against the provided user
        if self.enabled {
            match &self.release_type {
                ReleaseType::Global => {
                    return true;
                }
                ReleaseType::Limited(user_states) => {
                    match user_states.get(&ObjectId::parse_str(user).unwrap()) {
                        Some(user_state) => {
                            if !self.client_toggle {
                                return true;
                            }

                            if *user_state {
                                return true;
                            }
                        }
                        None => {}
                    }
                }
                ReleaseType::Percentage(_, user_states) => {
                    match user_states.get(&ObjectId::parse_str(user).unwrap()) {
                        Some(user_state) => {
                            if !self.client_toggle {
                                return true;
                            }

                            if *user_state {
                                return true;
                            }
                        }
                        None => {}
                    }
                }
            }
        }

        false
    }
}

impl Default for FeatureFlag {
    fn default() -> FeatureFlag {
        FeatureFlag {
            id: ObjectId::default(),
            name: "default_flag".to_string(),
            enabled: false,
            client_toggle: false,
            release_type: ReleaseType::Global,
        }
    }
}

///
/// # ReleaseType
/// Data object for a Feature Flag Release Type
/// 
/// Release types contain relevant information to the type of release
/// 
#[derive(Debug, Serialize, Deserialize)]
pub enum ReleaseType {
    /// Release is global, only controlled by the `FeatureFlag`s `enabled` property
    Global,
    /// Release is limited, limited to a specified list of `user_states`
    Limited(HashMap<ObjectId, bool>),
    /// Release is percentage, limited to a `percentage` of `user_states`
    Percentage (f32, HashMap<ObjectId, bool>),
}
