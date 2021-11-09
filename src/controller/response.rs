//! Response data structures for endpoints

use rocket::serde::{json::Json, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};

#[derive(Serialize, JsonSchema)]
pub struct FlagCheck {
  pub enabled: bool,
}

impl FlagCheck {
  pub async fn get_enabled() -> Option<Json<FlagCheck>> {
    Some(Json(FlagCheck { enabled: true }))
  }

  pub async fn get_disabled() -> Option<Json<FlagCheck>> {
    Some(Json(FlagCheck { enabled: false }))
  }
}

#[derive(Serialize, JsonSchema)]
pub struct Created {
  id: String,
}

impl Created {
  pub fn new(id: &str) -> Created {
    Created { id: id.to_string() }
  }
}
