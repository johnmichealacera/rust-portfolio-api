//! Read-only inspection script: dumps the live `personals` and `currentWork`
//! documents (filtered by USER_EMAIL) so copy decisions are grounded in what's
//! actually live, not just component fallback text.
//! Run with: cargo run --bin inspect_content
use std::env;

use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set");
    let user_email = env::var("USER_EMAIL").unwrap_or_else(|_| "default_value".to_string());
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("personal");

    for collection_name in ["personals", "currentwork", "skillsoverview", "socialmedias", "introductions"] {
        println!("\n=== {collection_name} ===");
        let collection = db.collection::<mongodb::bson::Document>(collection_name);
        let mut cursor = collection.find(doc! { "email": &user_email }, None).await?;
        while cursor.advance().await? {
            let doc = cursor.deserialize_current()?;
            println!("{:#}", doc);
        }
    }

    Ok(())
}
