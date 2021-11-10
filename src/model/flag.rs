//! Data model structures of the Feature Flag

use std::collections::HashMap;

use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

/// Data Object for a Feature Flag
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlag {
  /// Unique ID of the feature flag
  #[serde(alias = "_id", skip_serializing)]
  pub id: String,
  /// Name of the feature flag
  pub name: String,
  /// Unique ID of the product the feature flag belongs to
  pub product_id: String,
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
      id: "default_id".to_string(),
      name: "default_flag".to_string(),
      product_id: "default_product".to_string(),
      enabled: false,
      client_toggle: false,
      release_type: ReleaseType::Global,
    }
  }
}

impl FeatureFlag {
  /// Returns a `FeatureFlagBuilder` to eventually construct a `FeatureFlag`
  pub fn builder() -> FeatureFlagBuilder {
    FeatureFlagBuilder::new()
  }

  /// Evaluates the flag returning true if it is enabled and false otherwise
  ///
  /// Can optionally be provided a user to evaluate with
  ///
  /// # Parameters
  /// * **user** - *(optional)* User used to evaluate the flag with
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
        ReleaseType::Limited(user_states) => match user_states.get(user) {
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
        ReleaseType::Percentage(_, user_states) => match user_states.get(user) {
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
pub struct FeatureFlagBuilder {
  /// String generated my MongoDB
  pub id: String,
  /// Flag Name
  pub name: String,
  /// Unique ID of the product the feature flag belongs to
  pub product_id: String,
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
      product_id: default_flag.product_id,
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

  pub fn with_id(mut self, id: String) -> FeatureFlagBuilder {
    self.id = id;
    self
  }

  pub fn with_name(mut self, name: &str) -> FeatureFlagBuilder {
    self.name = name.to_string();
    self
  }

  pub fn with_product_id(mut self, product_id: &str) -> FeatureFlagBuilder {
    self.product_id = product_id.to_string();
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
      product_id: self.product_id,
      enabled: self.enabled,
      client_toggle: self.client_toggle,
      release_type: self.release_type,
    }
  }
}

/// Data object for a Feature Flag Release Type
///
/// Release types contain relevant information to the type of release
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ReleaseType {
  /// Release is global, only controlled by the `FeatureFlag`'s `enabled` property
  Global,
  /// Release is limited, limited to a specified `HashMap<String, bool>` where the key is a user ID and the value is
  /// the enabled status
  Limited(HashMap<String, bool>),
  /// Release is percentage, limited to a specified percentage of users contained in the specified `HashMap<String,
  /// bool>`
  Percentage(f32, HashMap<String, bool>),
}
