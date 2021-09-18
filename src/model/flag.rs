//!
//! Data model structures of the Feature Flag
//! 

use std::collections::HashMap;

pub struct FeatureFlag {
    pub id: usize,
    pub name: String,
    pub enabled: bool,
    pub client_toggle: bool,
    pub release_type: ReleaseType,
}

pub enum ReleaseType {
    Global,
    Limited {user_states: HashMap<usize, bool>},
    Percentage {percentage: f32, user_states: HashMap<usize, bool>},
}
