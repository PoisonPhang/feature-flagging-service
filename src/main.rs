//! API and server endpoints

#[macro_use]
extern crate rocket;

mod controller;
mod model;

use std::sync::{Arc, Mutex};

use rocket::http::{Cookie, CookieJar};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::swagger_ui::{self, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};

use controller::authentication::{AuthTokens, UserAuth};
use controller::database::ConnectionManager;
use controller::response::{Created, FlagCheck};
use model::flag::{FeatureFlag, ReleaseType, SpecSafeFeatureFlag};
use model::product::{Product, SpecSafeProduct};
use model::user::{AccountType, SpecSafeUser, User};

const USER_ID: &str = "user_id";
const AUTH_TOKEN: &str = "auth_token";

#[openapi(skip)]
#[get("/")]
async fn index() -> String {
  "Not 404, we just don't have a page yet".to_string()
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

/// Hoist a flag!
///
/// If the user is a `AccountType::Developer` then the flag is **enabled** globally
///
/// If the user is a `AccountType::Client` then the flag is **enabled** for that user.
/// The user will still need to have access to the flag
///
/// Returns 400 if something goes wrong, 202 otherwise
///
/// # Parameters
/// * **product_id** - Unique ID of the product
/// * **feature**    - Name of the feature
/// * **user_email** - email of the user hoisting the flag
#[openapi(tag = "Flags")]
#[patch("/hoist/<product_id>/<feature>/<user_email>")]
async fn hoist(
  product_id: &str,
  feature: &str,
  user_email: &str,
  database_connection: &State<ConnectionManager>,
) -> Result<status::Accepted<()>, status::BadRequest<()>> {
  let mut flag = match database_connection.get_feature_flag(product_id, feature).await {
    Some(flag) => flag,
    None => return Err(status::BadRequest(None)),
  };

  let flag_id = match flag.oid {
    Some(oid) => oid.to_hex(),
    None => return Err(status::BadRequest(None)),
  };

  let user_id: Option<String> = match database_connection.get_user(Some(user_email), None).await {
    Some(user) => match user.account_type {
      AccountType::Developer => None,
      AccountType::Client => match user.oid {
        Some(oid) => Some(oid.to_hex()),
        None => return Err(status::BadRequest(None)),
      },
    },
    None => return Err(status::BadRequest(None)),
  };

  flag.hoist(user_id);

  if database_connection.update_feature_flag(&flag_id, flag).await {
    return Ok(status::Accepted(None));
  }

  Err(status::BadRequest(None))
}

/// Lower a flag
///
/// If the user is a `AccountType::Developer` then the flag is **disabled** globally
///
/// If the user is a `AccountType::Client` then the flag is **disabled** for that user.
/// The user will still need to have access to the flag
///
/// Returns 400 if something goes wrong, 202 otherwise
///
/// # Parameters
/// * **product_id** - Unique ID of the product
/// * **feature**    - Name of the feature
/// * **user_email** - email of the user lowering the flag
#[openapi(tag = "Flags")]
#[patch("/lower/<product_id>/<feature>/<user_email>")]
async fn lower(
  product_id: &str,
  feature: &str,
  user_email: &str,
  database_connection: &State<ConnectionManager>,
) -> Result<status::Accepted<()>, status::BadRequest<()>> {
  let mut flag = match database_connection.get_feature_flag(product_id, feature).await {
    Some(flag) => flag,
    None => return Err(status::BadRequest(None)),
  };

  let flag_id = match flag.oid {
    Some(oid) => oid.to_hex(),
    None => return Err(status::BadRequest(None)),
  };

  let user_id: Option<String> = match database_connection.get_user(Some(user_email), None).await {
    Some(user) => match user.account_type {
      AccountType::Developer => None,
      AccountType::Client => match user.oid {
        Some(oid) => Some(oid.to_hex()),
        None => return Err(status::BadRequest(None)),
      },
    },
    None => return Err(status::BadRequest(None)),
  };

  flag.lower(user_id);

  if database_connection.update_feature_flag(&flag_id, flag).await {
    return Ok(status::Accepted(None));
  }

  Err(status::BadRequest(None))
}

/// Gets a product given a name
///
/// Will return 404 if no product with the given name is found
///
/// # Parameters
/// * **name** - Name of the product
#[openapi(tag = "Products")]
#[get("/get/product/<name>")]
async fn get_product(
  name: &str,
  database_connection: &State<ConnectionManager>,
) -> Result<Json<SpecSafeProduct>, status::NotFound<()>> {
  let product = match database_connection.get_product(name).await {
    Some(product) => product,
    None => return Err(status::NotFound(())),
  };

  Ok(Json(product.get_spec_safe_product()))
}

/// Gets all products that a user consumes
///
/// If no products are found - this will return an empty list
///
/// # Parameters
/// * **user_email** - email of a given user
#[openapi(tag = "Products")]
#[get("/get/products/<user_email>")]
async fn get_products(user_email: &str, database_connection: &State<ConnectionManager>) -> Json<Vec<SpecSafeProduct>> {
  let user = match database_connection.get_user(Some(user_email), None).await {
    Some(value) => value,
    None => return Json(vec![]),
  };

  let user_id = match user.oid {
    Some(oid) => oid,
    None => return Json(vec![]),
  };

  Json(
    database_connection
      .get_products(&user_id.to_hex())
      .await
      .iter()
      .map(|x| x.get_spec_safe_product())
      .collect::<Vec<SpecSafeProduct>>(),
  )
}

/// Gets a feature flag given a flag name and product ID
///
/// Will return 404 if no flag is found matching the search
///
/// # Parameters
/// * **name**       - name of the feature flag
/// * **product_id** - unique ID of the flag's product
#[openapi(tag = "Flags")]
#[get("/get/flag/<name>/<product_id>")]
async fn get_flag(
  name: &str,
  product_id: &str,
  database_connection: &State<ConnectionManager>,
) -> Result<Json<SpecSafeFeatureFlag>, status::NotFound<()>> {
  let flag = match database_connection.get_feature_flag(product_id, name).await {
    Some(flag) => flag,
    None => return Err(status::NotFound(())),
  };

  Ok(Json(flag.get_spec_safe_feature_flag()))
}

/// Gets all feature flags belonging to a product specified by product ID
///
/// Will return an empty list if no flags are found
///
/// # Paramaters
/// * **product_id** - unique ID of the product
#[openapi(tag = "Flags")]
#[get("/get/flags/<product_id>")]
async fn get_flags(product_id: &str, database_connection: &State<ConnectionManager>) -> Json<Vec<SpecSafeFeatureFlag>> {
  Json(
    database_connection
      .get_feature_flags(product_id)
      .await
      .iter()
      .map(|x| x.get_spec_safe_feature_flag())
      .collect::<Vec<SpecSafeFeatureFlag>>(),
  )
}

#[openapi(tag = "Users")]
#[get("/get/user/<user_id>")]
async fn get_user(
  user_id: &str,
  database_connection: &State<ConnectionManager>,
) -> Result<Json<SpecSafeUser>, status::NotFound<()>> {
  let user = match database_connection.get_user(None, Some(user_id)).await {
    Some(user) => user,
    None => return Err(status::NotFound(())),
  };

  Ok(Json(user.get_spec_safe_user()))
}

#[openapi(tag = "Users")]
#[get("/get/users/<account_type>")]
async fn get_users(
  account_type: Option<String>,
  database_connection: &State<ConnectionManager>,
) -> Json<Vec<SpecSafeUser>> {
  let users = match account_type {
    Some(account_type) => {
      database_connection
        .get_users(Some(AccountType::from(account_type)))
        .await
    }
    None => database_connection.get_users(None).await,
  };

  return Json(
    users
      .iter()
      .map(|x| x.get_spec_safe_user())
      .collect::<Vec<SpecSafeUser>>(),
  );
}

/// Create a product with a given name
///
/// Can provide a list of initial users (by user ID) for the product
///
/// # Parameters
/// * **name**  - Name of the new product
/// * **users** - List of initial users (send empty list if none are desired)
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

  let product_id = match product.oid {
    Some(oid) => oid,
    None => return Err(status::BadRequest(None)),
  };

  Ok(status::Created::new(format!("/get/product/{}", product.name)).body(Json(Created::new(&product_id.to_hex()))))
}

/// Create a flag with a given name, status, the `client_toggle` enum, and release type
///
/// The `client_toggle` enum determines if the flag can be toggled by clients
///
/// Leaving release type undefined will have it default to `Global`
///
/// # Parameters
/// * **name**          - Name of the new feature flag
/// * **product_id**    - Unique ID of product the flag belongs to
/// * **enabled**       - If the flag is enabled (true) or not (false)
/// * **client_toggle** - If clients can toggle flags on/off for themselves
/// * **release_type**  - Release type enum containing relevant data to the release type
#[openapi(tag = "Flags")]
#[post(
  "/create/flag/<name>/<product_id>/<enabled>/<client_toggle>",
  data = "<release_type>"
)]
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

  let flag_id = match flag.oid {
    Some(oid) => oid,
    None => return Err(status::BadRequest(None)),
  };

  Ok(
    status::Created::new(format!("/get/flag/{}/{}", flag.name, flag.product_id))
      .body(Json(Created::new(&flag_id.to_hex()))),
  )
}

/// Create a user with a given name, email, and password hash
///
/// # Parameters
/// * **account_type** - type of account
/// * **name**         - Name of the new user
/// * **email**        - Email address for the new user
/// * **hash**         - Hashed password of the new user
#[openapi(tag = "Users")]
#[post("/create/user/<name>/<email>/<hash>/<account_type>")]
async fn create_user(
  account_type: String,
  name: &str,
  email: &str,
  hash: &str,
  database_connection: &State<ConnectionManager>,
  _token_auth: UserAuth,
) -> Result<status::Created<Json<Created>>, status::BadRequest<()>> {
  let user_builder = User::builder()
    .with_name(name)
    .with_account_type(AccountType::from(account_type))
    .with_email(email)
    .with_password_hash(hash);

  let user = match database_connection.create_user(user_builder).await {
    Some(value) => value,
    None => return Err(status::BadRequest(None)),
  };

  let user_id = match user.oid {
    Some(oid) => oid,
    None => return Err(status::BadRequest(None)),
  };

  Ok(status::Created::new(format!("/get/user/{}", &user_id.to_hex())).body(Json(Created::new(&user_id.to_hex()))))
}

/// Login as a user
///
/// # Parameters
/// * **email** - email of the user being logged in
/// * **hash**  - Hashed password of the user being logged in
#[openapi(tag = "Users")]
#[get("/login/<email>/<hash>")]
async fn login(
  email: &str,
  hash: &str,
  database_connection: &State<ConnectionManager>,
  auth_tokens_mut: &State<Arc<Mutex<AuthTokens>>>,
  jar: &CookieJar<'_>,
) -> Result<status::Accepted<()>, status::BadRequest<String>> {
  let user = match database_connection.get_user(Some(email), None).await {
    Some(value) => value,
    None => return Err(status::BadRequest(Some(format!("User {} not found", email)))),
  };

  if user.password_hash == hash {
    let mut auth_tokens = match auth_tokens_mut.lock() {
      Ok(value) => value,
      Err(poisoned) => poisoned.into_inner(), // recover from poisoned mutex
    };

    let user_id = match user.oid {
      Some(oid) => oid,
      None => return Err(status::BadRequest(None)),
    };

    // Add cookies for user id and authentication token to request
    jar.add_private(Cookie::new(USER_ID, user_id.to_hex()));
    jar.add_private(Cookie::new(AUTH_TOKEN, auth_tokens.add_token(&user_id.to_hex())));

    return Ok(status::Accepted(None));
  }

  Err(status::BadRequest(Some("Incorrect password".to_string())))
}

#[openapi(tag = "Users")]
#[post("/logout")]
async fn logout(auth_tokens_mut: &State<Arc<Mutex<AuthTokens>>>, jar: &CookieJar<'_>,) -> Result<status::Accepted<()>, status::BadRequest<String>> {
    // Get user ID from request cookies
    let user_id = match jar.get_private(USER_ID) {
        Some(user_id) => user_id.value().to_string(),
        None => return Err(status::BadRequest(Some("Not logged in".to_string()))),
    };
    
    // Remove login cookies
    jar.remove_private(Cookie::named(USER_ID));
    jar.remove_private(Cookie::named(AUTH_TOKEN));

    let mut auth_tokens = match auth_tokens_mut.lock() {
        Ok(auth_tokens) => auth_tokens,
        Err(poisoned) => poisoned.into_inner(),
    };

    if auth_tokens.remove_token(&user_id) {
        return Ok(status::Accepted(None))
    } else {
        return Err(status::BadRequest(Some("Not logged into server".to_string())))
    }
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .manage(ConnectionManager::new())
    .manage(Arc::new(Mutex::new(AuthTokens::new()))) // Wrap in Arc<Mutex<T>> for thread safe mutability
    .mount(
      "/",
      openapi_get_routes![
        index,
        check,
        hoist,
        lower,
        get_product,
        get_products,
        get_flag,
        get_flags,
        get_user,
        get_users,
        create_product,
        create_flag,
        create_user,
        login,
        logout,
      ],
    )
    .mount(
      "/swagger-ui/",
      swagger_ui::make_swagger_ui(&SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
      }),
    )
}
