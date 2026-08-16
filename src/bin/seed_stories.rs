//! Upserts public story documents into `personal.stories`.
//! Run with: cargo run --bin seed_stories
//!
//! Video URLs live on each scene as Cloudinary delivery URLs. Include the
//! transform `f_mp4,vc_h264,q_auto:eco,w_960,c_limit,ac_none,fps_24` between
//! `/upload/` and the public id. Do not point scenes at files in the Vue repo.
use std::env;

use dotenv::dotenv;
use mongodb::{
    bson::{self, doc, Document},
    options::{ClientOptions, ReplaceOptions},
    Client,
};
use serde_json::Value;

const STORY_JSON: &str =
    include_str!("../../data/stories/the-boy-who-was-born-into-a-broken-world.json");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set");
    let user_email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let collection = client
        .database("personal")
        .collection::<Document>("stories");

    let mut story: Value = serde_json::from_str(STORY_JSON)?;
    let slug = story
        .get("slug")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    story["email"] = Value::String(user_email.clone());

    let document = bson::to_document(&story)?;
    let result = collection
        .replace_one(
            doc! { "email": &user_email, "slug": &slug },
            document,
            ReplaceOptions::builder().upsert(true).build(),
        )
        .await?;

    println!(
        "Upserted story \"{}\" — matched {}, modified {}, upserted={:?}",
        slug, result.matched_count, result.modified_count, result.upserted_id
    );
    Ok(())
}
