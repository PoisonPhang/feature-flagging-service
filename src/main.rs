//!
//! Currently being used for testing
//! 
#[macro_use] extern crate rocket;

mod model;
mod controller;

use rocket::State;
use mongodb::bson::doc;
use controller::database::ConnectionManager;

use model::user::{User, UserBuilder};

const FLAG_TRUE: &str = "1";
const FLAG_FALSE: &str = "0";

#[get("/")]
async fn index() -> String {
    format!("Not 404, we just don't have a page yet")
}

#[get("/check/<product>/<feature>/<user>")]
async fn check_with_user(product: &str, feature: &str, user: &str, database_connection: &State<ConnectionManager>) -> String {
    match database_connection.get_feature_flag(product, feature).await {
        Some(response) => {
            if response.evaluate(Some(user)) {
                return FLAG_TRUE.to_string()
            }
        },
        None => return FLAG_TRUE.to_string()
    }

    FLAG_FALSE.to_string()
}

#[get("/check/<product>/<feature>")]
async fn check(product: &str, feature: &str, database_connection: &State<ConnectionManager>) -> String {
    match database_connection.get_feature_flag(product, feature).await {
        Some(response) => {
            if response.evaluate(None) {
                return FLAG_TRUE.to_string()
            }
        },
        None => return FLAG_TRUE.to_string()
    }

    FLAG_FALSE.to_string()
}

#[post("/create/user/<name>/<email>/<hash>")]
async fn create_user(name: &str, email: &str, hash: &str, database_connection: &State<ConnectionManager>) -> String {
    let user_builder = User::builder()
        .with_name(name)
        .with_email(email)
        .with_password_hash(hash);

    let user = match database_connection.create_user(user_builder).await {
        Some(value) => value,
        None => return format!("Failed to create user: {}", name)
    };

    format!("User {} created with id {}", user.name, user.id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(ConnectionManager::new()).mount("/", routes![index, check, check_with_user])
}
