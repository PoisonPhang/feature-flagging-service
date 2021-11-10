//! Currently being used for testing

#[macro_use]
extern crate rocket;

mod controller;
mod model;

use std::sync::{Arc, Mutex};

use rocket::http::{Cookie, CookieJar};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

use controller::authentication::{AuthTokens, UserAuth};
use controller::database::ConnectionManager;
use controller::response::{Created, FlagCheck};
use model::flag::{FeatureFlag, ReleaseType};
use model::product::Product;
use model::user::User;

const USER_ID: &str = "user_id";
const AUTH_TOKEN: &str = "auth_token";

#[openapi(skip)]
#[get("/")]
async fn index() -> String {
  format!("Not 404, we just don't have a page yet")
}

/// Checks a product's flag to see if it is enabled
///
/// Optionally can provide a user for flags that use limited/percentage release
/// 
/// # Parameters
/// * **product_id** - Unique ID of the product that the feature flag belongs to
/// * **feature**    - Name of the feature flag
/// * **user**       - *(optional)* unique ID of the user to evaluate the flag with
#[openapi(tag = "Flags")]
#[get("/check/<product_id>/<feature>/with?<user>")]
async fn check(
  product_id: &str,
  feature: &str,
  user: Option<&str>,
  database_connection: &State<ConnectionManager>,
) -> Option<Json<FlagCheck>> {
  match database_connection.get_feature_flag(product_id, feature).await {
    Some(response) => {
      if response.evaluate(user) {
        return FlagCheck::get_enabled().await;
      }
    }
    None => return None,
  }

  FlagCheck::get_disabled().await
}

/// Create a product with a given name
///
/// Can provide a list of initial users (by user ID) for the product
#[openapi(tag = "Products")]
#[post("/create/product/<name>", data = "<users>")]
async fn create_product(
  name: &str,
  users: Json<Vec<String>>,
  database_connection: &State<ConnectionManager>,
  _token_auth: UserAuth,
) -> Result<status::Created<Json<Created>>, status::BadRequest<()>> {
  let product_builder = Product::builder().with_name(name).with_users(users.into_inner());

  let product = match database_connection.create_product(product_builder).await {
    Some(value) => value,
    None => return Err(status::BadRequest(None)),
  };

  Ok(status::Created::new("").body(Json(Created::new(&product.id))))
}

/// Create a flag with a given name, status, the `client_toggle` enum, and release type
///
/// The `client_toggle` enum determines if the flag can be toggled by clients
///
/// Leaving release type undefined will have it default to `Global`
#[openapi(tag = "Flags")]
#[post("/create/flag/<name>/<product_id>/<enabled>/<client_toggle>", data = "<release_type>")]
async fn create_flag(
  name: &str,
  product_id: &str,
  enabled: bool,
  client_toggle: bool,
  release_type: Json<ReleaseType>,
  database_connection: &State<ConnectionManager>,
  _token_auth: UserAuth,
) -> Result<status::Created<Json<Created>>, status::BadRequest<()>> {
  let flag_builder = FeatureFlag::builder()
    .with_name(name)
    .with_product_id(product_id)
    .with_enabled(enabled)
    .with_client_toggle(client_toggle)
    .with_release_type(release_type.into_inner());

  let flag = match database_connection.create_flag(flag_builder).await {
    Some(value) => value,
    None => return Err(status::BadRequest(None)),
  };

  Ok(status::Created::new("").body(Json(Created::new(&flag.id))))
}

/// Create a user with a given name, email, and password hash
#[openapi(tag = "Users")]
#[post("/create/user/<name>/<email>/<hash>")]
async fn create_user(
  name: &str,
  email: &str,
  hash: &str,
  database_connection: &State<ConnectionManager>,
  _token_auth: UserAuth,
) -> Result<status::Created<Json<Created>>, status::BadRequest<()>> {
  let user_builder = User::builder()
    .with_name(name)
    .with_email(email)
    .with_password_hash(hash);

  let user = match database_connection.create_user(user_builder).await {
    Some(value) => value,
    None => return Err(status::BadRequest(None)),
  };

  Ok(status::Created::new("").body(Json(Created::new(&user.id))))
}

/// Login as a user
#[openapi(tag = "Users")]
#[get("/login/<email>/<hash>")]
async fn login(
  email: &str,
  hash: &str,
  database_connection: &State<ConnectionManager>,
  auth_tokens_mut: &State<Arc<Mutex<AuthTokens>>>,
  jar: &CookieJar<'_>,
) -> Result<status::Accepted<()>, status::BadRequest<String>> {
  let user = match database_connection.get_user(email).await {
    Some(value) => value,
    None => return Err(status::BadRequest(Some(format!("User {} not found", email)))),
  };

  if user.password_hash == hash {
    let mut auth_tokens = match auth_tokens_mut.lock() {
      Ok(value) => value,
      Err(poisoned) => poisoned.into_inner(), // recover from poisoned mutex
    };

    // Add cookies for user id and authentication token to request
    jar.add_private(Cookie::new(USER_ID, user.id.clone()));
    jar.add_private(Cookie::new(AUTH_TOKEN, auth_tokens.add_token(&user.id)));

    return Ok(status::Accepted(None));
  }

  Err(status::BadRequest(Some("Incorrect password".to_string())))
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .manage(ConnectionManager::new())
    .manage(Arc::new(Mutex::new(AuthTokens::new()))) // Wrap in Arc<Mutex<T>> for thread safe mutability
    .mount(
      "/",
      openapi_get_routes![index, check, create_product, create_flag, create_user, login],
    )
    .mount(
      "/swagger-ui/",
      make_swagger_ui(&SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
      }),
    )
}
