//! Response data structures for endpoints

use mongodb::bson::oid::ObjectId;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct Created {
  id: ObjectId,
}

impl Created {
  pub fn new(id: ObjectId) -> Created {
    Created { id }
  }
}
