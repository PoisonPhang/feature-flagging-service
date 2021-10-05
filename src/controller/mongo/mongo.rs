//!
//! MongoDB connection management
//! 

use dotenv;
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::options::{ClientOptions, FindOptions};
use futures::stream::TryStreamExt;

use crate::model::flag::FeatureFlag;

pub async fn get_feature_flag(product: &str, flag_name: &str) -> Result<FeatureFlag, mongodb::error::Error> {
    let client: Client = get_client().await?;
    let mut feature = FeatureFlag::default();

    // TODO Replade &str with .env value 
    let db = client.database("data");
    let features_collection = db.collection::<FeatureFlag>("features");

    let filter = doc! { "name": flag_name, "product": product};
    let find_options = FindOptions::builder().sort(doc! {"name" : 1}).build();

    let mut cursor = features_collection.find(filter, find_options).await?;

    while let Some(feature_flag) = cursor.try_next().await? {
        feature = feature_flag;
    }

    Ok(feature)
}

async fn get_client() -> Result<Client, mongodb::error::Error> {
    dotenv::dotenv().ok();

    let connection_string = match dotenv::var("MONGOSTR") {
        Ok(value) => value,
        Err(e) => {
            panic!("Error getting MongoDB connection string (MONGOSTR): {:?}", e);
        }
    };

    let mut client_options = ClientOptions::parse(connection_string).await?;

    let client = Client::with_options(client_options)?;

    Ok(client)
}
