use dotenv;
use mongodb::{Client, options::ClientOptions};

pub async fn get_client() -> Result<Client, mongodb::error::Error> {
    dotenv::dotenv().ok();

    let connection_string = match dotenv::var("MONGOSTR") {
        Ok(value) => value,
        Err(e) => {
            println!("Error getting MongoDB connection string (MONGOSTR): {:?}", e);
        }
    };

    let mut client_options = ClientOptions::parse(connection_string).await?;

    let client = Client::with_options(client_options)?;

    Ok(client)
}
