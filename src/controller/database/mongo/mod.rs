//! MongoDB connection management

use dotenv;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error;
use mongodb::options::ClientOptions;
use mongodb::Client;

use crate::model::flag::{FeatureFlag, FeatureFlagBuilder};
use crate::model::product::{Product, ProductBuilder};
use crate::model::user::{AccountType, User, UserBuilder};

/// Given a product name, this will search for and return a fully constructed `Product` from MongoDB wrapped inside of a
/// `Result`.
///
/// If no product is found, will return the result of `Product::default()`.  
///
/// ## Result Error
/// `Result` can contain a MongoDB specific error
pub async fn get_product(product_name: &str) -> error::Result<Option<Product>> {
  let client = get_client().await?;

  let db = client.database("data");
  let product_collection = db.collection::<Product>("products");

  let filter = doc! { "name": product_name };

  product_collection.find_one(filter, None).await
}

/// Gets a `Vec<Product>` given a user_id
///
/// Returns all products consumed by the user
pub async fn get_products(user_id: Option<String>) -> error::Result<Vec<Product>> {
  let client = get_client().await?;
  let mut products: Vec<Product> = vec![];

  let db = client.database("data");
  let product_collection = db.collection::<Product>("products");

  let mut filter = doc!();

  if let Some(user_id) = user_id {
    filter.insert("users", user_id);
  }

  let mut cursor = product_collection.find(filter, None).await?;

  while let Some(product) = cursor.try_next().await? {
    products.push(product);
  }

  Ok(products)
}

/// Given a product name and flag name, this will search for and return a fully constructed `FeatureFlag` from MongoDB
/// wrapped inside of a `Result`.
///
/// If no feature flag is found, will return the result of `FeatureFlag::default()`.  
///
/// ## Result Error
/// `Result` can contain a MongoDB specific error
pub async fn get_feature_flag(product_id: &str, flag_name: &str) -> error::Result<Option<FeatureFlag>> {
  let client = get_client().await?;

  let db = client.database("data");
  let features_collection = db.collection::<FeatureFlag>("features");

  let filter = doc! { "name": flag_name, "product_id": product_id };

  features_collection.find_one(filter, None).await
}

/// Gets a `Vec<FeatureFlag>` given a product_id
///
/// Returns all feature flags belonging to the product
pub async fn get_feature_flags(product_id: &str) -> error::Result<Vec<FeatureFlag>> {
  let client = get_client().await?;
  let mut feature_flags: Vec<FeatureFlag> = vec![];

  let db = client.database("data");
  let features_collection = db.collection::<FeatureFlag>("features");

  let filter = doc! {"product_id": product_id};

  let mut cursor = features_collection.find(filter, None).await?;

  while let Some(feature_flag) = cursor.try_next().await? {
    feature_flags.push(feature_flag);
  }

  Ok(feature_flags)
}

/// Updates a feature_flag of the given ID with the `updated` `FeatureFlag` struct
///
/// Returns a result indicating success
pub async fn update_feature_flag(feature_flag_id: ObjectId, updated: FeatureFlag) -> error::Result<()> {
  let client = get_client().await?;

  let db = client.database("data");
  let features_collection = db.collection::<FeatureFlag>("features");

  let query = doc! {"_id": feature_flag_id};

  features_collection.replace_one(query, updated, None).await?;

  Ok(())
}

/// Given a user email, this will search for and return a fully constructed `User` from MongoDB wrapped inside of a
/// `Result`.
///
/// If no user is found, will return the result of `User::default()`.  
///
/// ## Result Error
/// `Result` can contain a MongoDB specific error
pub async fn get_user(user_email: Option<&str>, user_id: Option<&str>) -> error::Result<Option<User>> {
  let client = get_client().await?;

  let db = client.database("data");
  let user_collection = db.collection::<User>("users");

  let mut filter = doc!();

  if let Some(email) = user_email {
    filter.insert("email", email);
  }

  if let Some(id) = user_id {
    filter.insert("_id", ObjectId::parse_str(id).unwrap_or_default());
  }

  user_collection.find_one(filter, None).await
}

/// Gets a `Vec<User>` optinally given an account_type
pub async fn get_users(account_type: Option<AccountType>) -> error::Result<Vec<User>> {
  let client = get_client().await?;
  let mut users: Vec<User> = vec![];

  let db = client.database("data");
  let user_collection = db.collection::<User>("users");

  let mut filter = doc!();

  if let Some(account_type) = &account_type {
    filter.insert("account_type", account_type);
  }

  let mut cursor = user_collection.find(filter, None).await?;

  while let Some(user) = cursor.try_next().await? {
    users.push(user);
  }

  Ok(users)
}

/// Creates a product given a builder and returns a fully constructed product
pub async fn create_product(product_builder: ProductBuilder) -> error::Result<Product> {
  let client = get_client().await?;

  let db = client.database("data");
  let products_collection = db.collection::<Product>("products");

  let product_id = products_collection
    .insert_one(product_builder.clone().build(), None)
    .await?
    .inserted_id
    .as_object_id()
    .unwrap_or_default();

  let product = product_builder.with_oid(product_id).build();

  Ok(product)
}

/// Creates a new feature flag given a builder and returns a fully constructed flag
pub async fn create_flag(flag_builder: FeatureFlagBuilder) -> error::Result<FeatureFlag> {
  let client = get_client().await?;

  let db = client.database("data");
  let features_collection = db.collection::<FeatureFlag>("features");

  let flag_id = features_collection
    .insert_one(flag_builder.clone().build(), None)
    .await?
    .inserted_id
    .as_object_id()
    .unwrap_or_default();

  let flag = flag_builder.with_oid(flag_id).build();

  Ok(flag)
}

/// Given a `UserBuilder`, this will attempt to create a new `User` and insert them into the database.
///
/// The `User` returned inside of the `Result` will contain the ObjectId generated by MongoDB
///
/// ## Result Error
/// `Result` can contain a MongoDB specific error
pub async fn create_user(user_builder: UserBuilder) -> error::Result<User> {
  let client = get_client().await?;

  let db = client.database("data");
  let user_collection = db.collection::<User>("users");

  let user_id = user_collection
    .insert_one(user_builder.clone().build(), None)
    .await?
    .inserted_id
    .as_object_id()
    .unwrap_or_default();

  let user = user_builder.with_oid(user_id).build();

  Ok(user)
}

async fn get_client() -> error::Result<Client> {
  dotenv::dotenv().ok();

  let connection_string = match dotenv::var("MONGO_STR") {
    Ok(value) => value,
    Err(e) => {
      panic!("Error getting MongoDB connection string (MONGO_STR): {:?}", e);
    }
  };

  let client_options = ClientOptions::parse(connection_string).await?;

  let client = Client::with_options(client_options)?;

  Ok(client)
}
