//! Response data structures for endpoints

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
pub struct FlagCheck {
    pub enabled: bool,
}

impl FlagCheck {
    pub async fn get_enabled() -> Option<Json<FlagCheck>> {
        Some(Json(FlagCheck { enabled: true }))
    }

    pub async fn get_disabled() -> Option<Json<FlagCheck>> {
        Some(Json(FlagCheck {enabled: false}))
    }
}
