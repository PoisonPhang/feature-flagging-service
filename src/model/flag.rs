//!
//! Data model structures of the Feature Flag
//! 

use std::collections::HashMap;

///
/// # FeatureFlag
/// 
/// Data Object for a Feature Flag
/// 
pub struct FeatureFlag {
    /// Flag Name
    pub name: String,
    /// Global enabled status of the flag (trumps other statuses)
    pub enabled: bool,
    /// If client toggles are enabled
    pub client_toggle: bool,
    /// Type of release and relevant data
    pub release_type: ReleaseType,
}

///
/// # ReleaseType
/// Data object for a Feature Flag Release Type
/// 
/// Release types contain relevant information to the type of release
/// 
pub enum ReleaseType {
    /// Release is global, only controlled by the `FeatureFlag`s `enabled` property
    Global,
    /// Release is limited, limited to a specified list of `user_states`
    Limited {user_states: HashMap<usize, bool>},
    /// Release is percentage, limited to a `percentage` of `user_states`
    Percentage {percentage: f32, user_states: HashMap<usize, bool>},
}
