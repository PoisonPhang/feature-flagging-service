//! Response data structures for endpoints

use rocket::serde::{json::Json, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};

/// Response from `/check/...` routes that will state if a flag is enabled or not
#[derive(Serialize, JsonSchema)]
pub struct FlagCheck {
  /// Status of the flag
  pub enabled: bool,
}

impl FlagCheck {
  /// Creates a `FlagCheck` with an enabled status
  pub async fn get_enabled() -> Option<Json<FlagCheck>> {
    Some(Json(FlagCheck { enabled: true }))
  }

  /// Creates a `FlagCheck` with an disabled status
  pub async fn get_disabled() -> Option<Json<FlagCheck>> {
    Some(Json(FlagCheck { enabled: false }))
  }
}

/// Response from `/create/...` routes containing the unique ID generated for the object/record
#[derive(Serialize, JsonSchema)]
pub struct Created {
  /// Unique ID generated for the object/record
  pub id: String,
}

impl Created {
  /// Creates and returns a new `Created` with the given string (`&str`)
  pub fn new(id: &str) -> Created {
    Created { id: id.to_string() }
  }
}
