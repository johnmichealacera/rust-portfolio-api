//! Manual, one-off seed script: rewrites the live hero tagline (personals.jobDescription)
//! from agency/freelance-coded copy to backend-forward, hiring-oriented copy.
//! Run with: cargo run --bin seed_hero_copy
use std::env;

use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};

const NEW_JOB_DESCRIPTION: &str = "I'm a backend-focused full-stack engineer with 6+ years building production workflow systems — APIs, data models, access control, and integrations — for product companies and real institutions. Open to senior backend/full-stack roles, and select project collaborations.";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set");
    let user_email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("personal");
    let collection = db.collection::<mongodb::bson::Document>("personals");

    let result = collection
        .update_one(
            doc! { "email": &user_email },
            doc! { "$set": { "jobDescription": NEW_JOB_DESCRIPTION } },
            None,
        )
        .await?;

    println!("Matched {} document(s), modified {}.", result.matched_count, result.modified_count);
    Ok(())
}
