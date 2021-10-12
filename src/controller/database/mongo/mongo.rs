//!
//! MongoDB connection management
//! 

use dotenv;
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::options::{ClientOptions, FindOptions};
use futures::stream::TryStreamExt;

use crate::model::flag::FeatureFlag;
use crate::model::product::Product;
use crate::model::user::User;

pub async fn get_product(product_name: &str) -> Result<Product, mongodb::error::Error> {
    let client = get_client().await?;
    let mut product = Product::default();

    let db = client.database("data");
    let product_collection = db.collection::<Product>("products");

    let filter = doc! { "name": product_name };
    let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();

    let mut cursor = product_collection.find(filter, find_options).await?;

    while let Some(product_found) = cursor.try_next().await? {
        product = product_found;
    }

    Ok(product)
}

pub async fn get_feature_flag(product: &str, flag_name: &str) -> Result<FeatureFlag, mongodb::error::Error> {
    let client = get_client().await?;
    let mut feature = FeatureFlag::default();

    let db = client.database("data");
    let features_collection = db.collection::<FeatureFlag>("features");

    let filter = doc! { "name": flag_name, "product": product };
    let find_options = FindOptions::builder().sort(doc! {"name": 1 }).build();

    let mut cursor = features_collection.find(filter, find_options).await?;

    while let Some(feature_found) = cursor.try_next().await? {
        feature = feature_found;
    }

    Ok(feature)
}

pub async fn get_user(user_email: &str) -> Result<User, mongodb::error::Error> {
    let client = get_client().await?;
    let mut user = User::default();

    let db = client.database("data");
    let user_collection = db.collection::<User>("users");

    let filter = doc! {"email": user_email };
    let find_options = FindOptions::builder().sort(doc! { "email": 1}).build();

    let mut cursor = user_collection.find(filter, find_options).await?;

    while let Some(user_found) = cursor.try_next().await? {
        user = user_found;
    }

    Ok(user)
}

async fn get_client() -> Result<Client, mongodb::error::Error> {
    dotenv::dotenv().ok();

    let connection_string = match dotenv::var("MONGO_STR") {
        Ok(value) => value,
        Err(e) => {
            panic!("Error getting MongoDB connection string (MONGOSTR): {:?}", e);
        }
    };

    let client_options = ClientOptions::parse(connection_string).await?;

    let client = Client::with_options(client_options)?;

    Ok(client)
}
