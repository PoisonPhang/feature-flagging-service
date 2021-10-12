//!
//! Currently being used for testing
//! 
#[macro_use] extern crate rocket;
use rocket::State;

use mongodb::{bson::doc, bson::oid::ObjectId, Client, options::FindOptions};
use futures::stream::TryStreamExt;

mod model;
mod controller;

use model::flag::ReleaseType;
use controller::database::ConnectionManager;

const FLAG_TRUE: &str = "1";
const FLAG_FALSE: &str = "0";

#[get("/")]
async fn index() -> String {
    "Not 404, we just don't have a page yet".to_string()
}

#[get("/check/<product>/<feature>/<user>")]
async fn check(product: &str, feature: &str, user: &str, database_connection: &State<ConnectionManager>) -> String {
    match database_connection.get_feature_flag(product, feature).await {
        Some(response) => {
            if response.evaluate(user) {
                return FLAG_TRUE.to_string()
            }
        },
        None => return FLAG_TRUE.to_string()
    }

    FLAG_FALSE.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(ConnectionManager::new()).mount("/", routes![index, check])
}

/*

use std::collections::HashMap;
use std::{env, fs};

use bson::Bson;

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};

use model::flag::{FeatureFlag, ReleaseType};
use model::product::Product;
use model::user::User;


const CREDS_FILE: &str = "database.creds";

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let connection_string = fs::read_to_string(CREDS_FILE).expect("Something went wrong reading creds file");
    let client_options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("data");
    let products = db.collection::<Product>("products");
    let features = db.collection::<FeatureFlag>("features");
    let users = db.collection::<User>("users");

    let test_user = User::new("test".to_string(), "user@test.org".to_string(), "abcapplefarm123".to_string());
    let test_flag = FeatureFlag::new("test:test_flag".to_string(), true, false, ReleaseType::Global);
    let mut test_product = Product::new("test".to_string(), vec!());

    let user_id = users.insert_one(test_user, None).await?.inserted_id;
    println!("Inserted user with ObjectId: {}", user_id);

    let feature_id = features.insert_one(test_flag, None).await?.inserted_id;
    println!("Inserted flag with ObjectId: {}", feature_id);

    test_product.features.push(feature_id.as_object_id().unwrap());

    let product_id = products.insert_one(test_product, None).await?.inserted_id;
    println!("Inserted flag with ObjectId: {}", product_id);

    Ok(())
}
*/
