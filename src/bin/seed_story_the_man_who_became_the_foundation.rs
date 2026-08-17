//! Upserts the "The Man Who Became the Foundation" story into `personal.stories`.
//! Run with: cargo run --bin seed_story_the_man_who_became_the_foundation
//!
//! Scenes are seeded with `video: null` / `image: null` placeholders — the
//! frontend falls back to a mood-keyed gradient for any scene missing media.
//! Once clips exist, follow the same Cloudinary + transform convention as
//! `seed_stories.rs` and re-run this binary to update scenes in place.
use std::env;

use dotenv::dotenv;
use mongodb::{
    bson::{self, doc, Document},
    options::{ClientOptions, ReplaceOptions},
    Client,
};
use serde_json::Value;

const STORY_JSON: &str =
    include_str!("../../data/stories/the-man-who-became-the-foundation.json");

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
