//! Currently being used for testing

#[macro_use]
extern crate rocket;

mod controller;
mod model;

use std::sync::{Arc, Mutex};

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::State;

use controller::authentication::{AuthTokens, UserAuth};
use controller::database::ConnectionManager;
use controller::response::FlagCheck;
use model::flag::{FeatureFlag, ReleaseType};
use model::product::Product;
use model::user::User;

const USER_ID: &str = "user_id";
const AUTH_TOKEN: &str = "auth_token";

#[get("/")]
async fn index() -> String {
  format!("Not 404, we just don't have a page yet")
}

#[get("/check/<product>/<feature>/with?<user>")]
async fn check(
  product: &str,
  feature: &str,
  user: Option<&str>,
  database_connection: &State<ConnectionManager>,
) -> Option<Json<FlagCheck>> {
  match database_connection.get_feature_flag(product, feature).await {
    Some(response) => {
      if response.evaluate(user) {
        return FlagCheck::get_enabled().await;
      }
    }
    None => return None,
  }

  FlagCheck::get_disabled().await
}

#[post("/create/product/<name>", data = "<users>")]
async fn create_product(
  name: &str,
  users: Json<Vec<ObjectId>>,
  database_connection: &State<ConnectionManager>,
  _token_auth: UserAuth,
) -> String {
  let product_builder = Product::builder().with_name(name).with_users(users.into_inner());

  let product = match database_connection.create_product(product_builder).await {
    Some(value) => value,
    None => return format!("Failed to create product: {}", name),
  };

  format!("Created product {} with id {}", name, product.id)
}

#[post("/create/flag/<name>/<enabled>/<client_toggle>", data = "<release_type>")]
async fn create_flag(
  name: &str,
  enabled: bool,
  client_toggle: bool,
  release_type: Json<ReleaseType>,
  database_connection: &State<ConnectionManager>,
  _token_auth: UserAuth,
) -> String {
  let flag_builder = FeatureFlag::builder()
    .with_name(name)
    .with_enabled(enabled)
    .with_client_toggle(client_toggle)
    .with_release_type(release_type.into_inner());

  let flag = match database_connection.create_flag(flag_builder).await {
    Some(value) => value,
    None => return format!("Failed to create flag: {}", name),
  };

  format!("Created flag {} with id {}", name, flag.id)
}

#[post("/create/user/<name>/<email>/<hash>")]
async fn create_user(
  name: &str,
  email: &str,
  hash: &str,
  database_connection: &State<ConnectionManager>,
  _token_auth: UserAuth,
) -> String {
  let user_builder = User::builder()
    .with_name(name)
    .with_email(email)
    .with_password_hash(hash);

  let user = match database_connection.create_user(user_builder).await {
    Some(value) => value,
    None => return format!("Failed to create user: {}", name),
  };

  format!("User {} created with id {}", user.name, user.id)
}

#[get("/login/<email>/<hash>")]
async fn login(
  email: &str,
  hash: &str,
  database_connection: &State<ConnectionManager>,
  auth_tokens_mut: &State<Arc<Mutex<AuthTokens>>>,
  jar: &CookieJar<'_>,
) -> String {
  let user = match database_connection.get_user(email).await {
    Some(value) => value,
    None => return format!("User {} not found", email),
  };

  if user.password_hash == hash {
    let mut auth_tokens = match auth_tokens_mut.lock() {
      Ok(value) => value,
      Err(poisoned) => poisoned.into_inner(), // recover from poisoned mutex
    };

    // Add cookies for user id and authentication token to request
    jar.add_private(Cookie::new(USER_ID, user.id.to_hex()));
    jar.add_private(Cookie::new(AUTH_TOKEN, auth_tokens.add_token(user.id)));

    return format!("Login success");
  }

  "Incorrect password".to_string()
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .manage(ConnectionManager::new())
    .manage(Arc::new(Mutex::new(AuthTokens::new()))) // Wrap in Arc<Mutex<T>> for thread safe mutability
    .mount(
      "/",
      routes![index, check, create_product, create_flag, create_user, login],
    )
}
