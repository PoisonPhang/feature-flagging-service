//! Data model structures of the Feature Flag

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

/// Data Object for a Feature Flag
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlag {
  /// Unique ID of the feature flag
  #[serde(alias = "_id", skip_serializing_if = "Option::is_none")]
  pub oid: Option<ObjectId>,
  /// Name of the feature flag
  pub name: String,
  /// Unique ID of the product the feature flag belongs to
  pub product_id: String,
  /// Global enabled status of the flag (false trumps other statuses)
  pub enabled: bool,
  /// If client toggles are enabled
  pub client_toggle: bool,
  /// List of all users who've disabled the feature
  pub disabled_for: Vec<String>,
  /// Type of release and relevant data
  pub release_type: ReleaseType,
}

impl Default for FeatureFlag {
  fn default() -> FeatureFlag {
    FeatureFlag {
      oid: Default::default(),
      name: "default_flag".to_string(),
      product_id: "default_product".to_string(),
      enabled: false,
      client_toggle: false,
      disabled_for: vec![],
      release_type: ReleaseType::Global,
    }
  }
}

impl FeatureFlag {
  /// Returns a `FeatureFlagBuilder` to eventually construct a `FeatureFlag`
  pub fn builder() -> FeatureFlagBuilder {
    FeatureFlagBuilder::new()
  }

  pub fn hoist(&mut self, user_id: Option<String>) {
    match user_id {
      Some(user_id) => self.disabled_for.retain(|x| x != &user_id),
      None => self.enabled = true,
    }
  }

  pub fn lower(&mut self, user_id: Option<String>) {
    match user_id {
      Some(user_id) => self.disabled_for.push(user_id),
      None => self.enabled = false,
    }
  }

  /// Evaluates the flag returning true if it is enabled and false otherwise
  ///
  /// Can optionally be provided a user to evaluate with
  ///
  /// # Parameters
  /// * **user_id** - *(optional)* User used to evaluate the flag with
  pub fn evaluate(&self, user_id: Option<&str>) -> bool {
    if !self.enabled {
      return false;
    }

    match &self.release_type {
      ReleaseType::Global => match user_id {
        Some(user_id) => {
          if self.disabled_for.contains(&user_id.to_string()) {
            return false;
          } else {
            return self.enabled;
          }
        }
        None => return self.enabled,
      },
      ReleaseType::Limited(allowlist) => match user_id {
        Some(user_id) => {
          if self.disabled_for.contains(&user_id.to_string()) {
            return false;
          }
          if allowlist.contains(&user_id.to_string()) {
            return self.enabled;
          }
        }
        None => return false,
      },
      ReleaseType::Percentage(_, allowlist) => match user_id {
        Some(user_id) => {
          if self.disabled_for.contains(&user_id.to_string()) {
            return false;
          }
          if allowlist.contains(&user_id.to_string()) {
            return self.enabled;
          }
        }
        None => return false,
      },
    }

    false
  }

  pub fn get_spec_safe_feature_flag(&self) -> SpecSafeFeatureFlag {
    SpecSafeFeatureFlag {
      oid: match self.oid {
        Some(oid) => oid.to_hex(),
        None => ObjectId::default().to_hex(),
      },
      name: self.name.clone(),
      product_id: self.product_id.clone(),
      enabled: self.enabled,
      client_toggle: self.client_toggle,
      release_type: self.release_type.clone(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SpecSafeFeatureFlag {
  // Unique ID of the feature flag
  pub oid: String,
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

#[derive(Clone)]
pub struct FeatureFlagBuilder {
  /// String generated my MongoDB
  pub oid: Option<ObjectId>,
  /// Flag Name
  pub name: String,
  /// Unique ID of the product the feature flag belongs to
  pub product_id: String,
  /// Global enabled status of the flag (false trumps other statuses)
  pub enabled: bool,
  /// If client toggles are enabled
  pub client_toggle: bool,
  /// List of all users who've disabled the feature
  pub disabled_for: Vec<String>,
  /// Type of release and relevant data
  pub release_type: ReleaseType,
}

impl Default for FeatureFlagBuilder {
  fn default() -> FeatureFlagBuilder {
    let default_flag = FeatureFlag::default();

    FeatureFlagBuilder {
      oid: default_flag.oid,
      name: default_flag.name,
      product_id: default_flag.product_id,
      enabled: default_flag.enabled,
      client_toggle: default_flag.client_toggle,
      disabled_for: default_flag.disabled_for,
      release_type: default_flag.release_type,
    }
  }
}

impl FeatureFlagBuilder {
  fn new() -> FeatureFlagBuilder {
    FeatureFlagBuilder::default()
  }

  pub fn with_oid(mut self, oid: ObjectId) -> FeatureFlagBuilder {
    self.oid = Some(oid);
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

  pub fn with_disabled_for(mut self, disabled_for: Vec<String>) -> FeatureFlagBuilder {
    self.disabled_for = disabled_for;
    self
  }

  pub fn with_release_type(mut self, release_type: ReleaseType) -> FeatureFlagBuilder {
    self.release_type = release_type;
    self
  }

  pub fn build(self) -> FeatureFlag {
    FeatureFlag {
      oid: self.oid,
      name: self.name,
      product_id: self.product_id,
      enabled: self.enabled,
      client_toggle: self.client_toggle,
      disabled_for: self.disabled_for,
      release_type: self.release_type,
    }
  }
}

/// Data object for a Feature Flag Release Type
///
/// Release types contain relevant information to the type of release
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ReleaseType {
  /// Release is global
  Global,
  /// Release is limited, contains an allowlist of users
  Limited(Vec<String>),
  /// Release is percentage, contains a percentage and allowlist
  Percentage(f32, Vec<String>),
}
