//!
//! Data model structures of the Feature Flag
//! 

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

///
/// # FeatureFlag
/// 
/// Data Object for a Feature Flag
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlag {
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
    ///
    /// # FeatureFlag::new()
    /// Creates and returns a new `FeatureFlag` with the provided fields
    /// 
    /// ## Example
    /// ```
    /// let flag = FeatureFlag::new("example:flag_name".to_string(), true, false, ReleaseType::Global);
    /// ```
    pub fn new(name: String, enabled: bool, client_toggle: bool, release_type: ReleaseType) -> FeatureFlag {
        FeatureFlag {
            name,
            enabled,
            client_toggle,
            release_type,
        }
    }

    pub fn evaluate(&self, user: &str) -> bool {
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

        print!("Returning false\n");
        false
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
