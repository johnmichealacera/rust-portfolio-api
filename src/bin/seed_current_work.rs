//! Manual, one-off seed script: fixes ownership blur in currentwork.description —
//! previously mixed genuine Sportality bullets with independent PH institutional
//! work in one list, implying they were the same employer's work.
//! Run with: cargo run --bin seed_current_work
use std::env;

use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};

const DESCRIPTION: &[&str] = &[
    "Fullstack engineering at Sportality — backend API design, database modeling, and frontend delivery across the platform.",
    "Own REST API integrations and performance optimization work across the product.",
    "Independently building government and cooperative platforms for local institutions in the Philippines, alongside this role.",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set");
    let user_email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("personal");
    let collection = db.collection::<mongodb::bson::Document>("currentwork");

    let result = collection
        .update_one(
            doc! { "email": &user_email },
            doc! { "$set": { "description": DESCRIPTION } },
            None,
        )
        .await?;

    println!("Matched {} document(s), modified {}.", result.matched_count, result.modified_count);
    Ok(())
}
