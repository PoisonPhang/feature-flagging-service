//!
//! Data model structures of the Feature Flag

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

///
/// Data Object for a Feature Flag
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

impl FeatureFlag {
  pub fn evaluate(&self, user: Option<&str>) -> bool {
    // If no user is provided, only evaluate Global release type as potentially true
    let user = match user {
      Some(value) => value,
      None => {
        if self.enabled {
          match &self.release_type {
            ReleaseType::Global => return true,
            _ => return false,
          }
        }
        return false;
      }
    };

    // Evaluate each release type against the provided user
    if self.enabled {
      match &self.release_type {
        ReleaseType::Global => {
          return true;
        }
        ReleaseType::Limited(user_states) => match user_states.get(&ObjectId::parse_str(user).unwrap()) {
          Some(user_state) => {
            if !self.client_toggle {
              return true;
            }

            if *user_state {
              return true;
            }
          }
          None => {}
        },
        ReleaseType::Percentage(_, user_states) => match user_states.get(&ObjectId::parse_str(user).unwrap()) {
          Some(user_state) => {
            if !self.client_toggle {
              return true;
            }

            if *user_state {
              return true;
            }
          }
          None => {}
        },
      }
    }

    false
  }
}

#[derive(Clone)]
struct FeatureFlagBuilder {
  /// ObjectID generated my MongoDB
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

impl Default for FeatureFlagBuilder {
  fn default() -> FeatureFlagBuilder {
    let default_flag = FeatureFlag::default();

    FeatureFlagBuilder {
      id: default_flag.id,
      name: default_flag.name,
      enabled: default_flag.enabled,
      client_toggle: default_flag.client_toggle,
      release_type: default_flag.release_type,
    }
  }
}

impl FeatureFlagBuilder {
  fn new() -> FeatureFlagBuilder {
    FeatureFlagBuilder::default()
  }

  pub fn with_id(mut self, id: ObjectId) -> FeatureFlagBuilder {
    self.id = id;
    self
  }

  pub fn with_name(mut self, name: String) -> FeatureFlagBuilder {
    self.name = name;
    self
  }

  pub fn with_enabled(mut self, enabled: bool) -> FeatureFlagBuilder {
    self.enabled = enabled;
    self
  }

  pub fn with_client_toggle(mut self, client_toggle: bool) -> FeatureFlagBuilder {
    self.client_toggle = client_toggle;
    self
  }

  pub fn with_release_type(mut self, release_type: ReleaseType) -> FeatureFlagBuilder {
    self.release_type = release_type;
    self
  }

  pub fn build(self) -> FeatureFlag {
    FeatureFlag {
      id: self.id,
      name: self.name,
      enabled: self.enabled,
      client_toggle: self.client_toggle,
      release_type: self.release_type,
    }
  }
}

///
/// # ReleaseType
/// Data object for a Feature Flag Release Type
///
/// Release types contain relevant information to the type of release
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReleaseType {
  /// Release is global, only controlled by the `FeatureFlag`s `enabled` property
  Global,
  /// Release is limited, limited to a specified list of `user_states`
  Limited(HashMap<ObjectId, bool>),
  /// Release is percentage, limited to a `percentage` of `user_states`
  Percentage(f32, HashMap<ObjectId, bool>),
}
