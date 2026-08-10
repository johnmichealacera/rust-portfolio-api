//! Manual, one-off seed script: reorders hero expertise chips (backend-first)
//! and replaces the vague "Dedicated Performer" chip with "Systems Builder",
//! matching the hero badge copy already used elsewhere on the site.
//! Run with: cargo run --bin seed_skills_overview
use std::env;

use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};

struct ChipUpdate {
    match_title: &'static str,
    new_title: &'static str,
    sort_order: i32,
}

const CHIPS: &[ChipUpdate] = &[
    ChipUpdate { match_title: "Backend API Engineer", new_title: "Backend API Engineer", sort_order: 1 },
    ChipUpdate { match_title: "Dedicated Performer", new_title: "Systems Builder", sort_order: 2 },
    ChipUpdate { match_title: "Data Architect", new_title: "Data Architect", sort_order: 3 },
    ChipUpdate { match_title: "Web Development", new_title: "Web Development", sort_order: 4 },
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set");
    let user_email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("personal");
    let collection = db.collection::<mongodb::bson::Document>("skillsoverview");

    for chip in CHIPS {
        let result = collection
            .update_one(
                doc! { "email": &user_email, "title": chip.match_title },
                doc! { "$set": { "title": chip.new_title, "sortOrder": chip.sort_order } },
                None,
            )
            .await?;

        if result.matched_count == 0 {
            eprintln!("No chip found matching \"{}\" — skipped.", chip.match_title);
        } else {
            println!("Updated \"{}\" -> \"{}\" (sortOrder {})", chip.match_title, chip.new_title, chip.sort_order);
        }
    }

    Ok(())
}
