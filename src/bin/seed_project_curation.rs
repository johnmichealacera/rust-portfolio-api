//! Manual, one-off seed script: sets project `tier` (flagship/secondary/experiment
//! for portfolio curation) and `githubUrl` (only where a public repo was confidently
//! matched by name — left blank rather than guessed) for all 15 projects.
//! Run with: cargo run --bin seed_project_curation
use std::env;

use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client,
};

struct ProjectCuration {
    oid: &'static str,
    title: &'static str,
    tier: &'static str,
    github_url: &'static str,
}

const PROJECTS: &[ProjectCuration] = &[
    ProjectCuration { oid: "63d8b3c2af25d5ba7e2fe8bf", title: "HappyChat", tier: "experiment", github_url: "" },
    ProjectCuration { oid: "63d8bef70a798e287c123af5", title: "Javascript Bird Hunter", tier: "experiment", github_url: "" },
    ProjectCuration { oid: "645f74b3f4283c07c6213b1d", title: "DotaMate", tier: "experiment", github_url: "https://github.com/johnmichealacera/dota2-companion" },
    ProjectCuration { oid: "645f74dbf4283c07c6213b1e", title: "ImageScanner", tier: "experiment", github_url: "https://github.com/johnmichealacera/tensor-flow-app" },
    ProjectCuration { oid: "645f753ff4283c07c6213b1f", title: "Taskify", tier: "experiment", github_url: "" },
    ProjectCuration { oid: "66376b9f2ecc465531439667", title: "Help App Admin Dashboard", tier: "secondary", github_url: "https://github.com/johnmichealacera/help-app-admin" },
    ProjectCuration { oid: "66376f0ff9ca146bbdf6afc4", title: "CitizenCare Mobile App", tier: "secondary", github_url: "https://github.com/johnmichealacera/help-app-react-native" },
    ProjectCuration { oid: "66e3ae19a3b3b15b4476c806", title: "Soroy sa Socorro Website", tier: "secondary", github_url: "" },
    ProjectCuration { oid: "6832d053b6d454632088c95d", title: "Skill Match", tier: "secondary", github_url: "https://github.com/johnmichealacera/skill-match" },
    ProjectCuration { oid: "6857909cf1ddb6fd2c61422a", title: "CSS NC2 Companion", tier: "secondary", github_url: "https://github.com/johnmichealacera/css-nc2-training-companion" },
    ProjectCuration { oid: "688ac70b3b332b887d54ae45", title: "MD Events and Services", tier: "secondary", github_url: "" },
    ProjectCuration { oid: "69f6ae7307c68ed4285398be", title: "Unified Government System", tier: "flagship", github_url: "" },
    ProjectCuration { oid: "69f6ae7307c68ed4285398bd", title: "Pageant Tabulation System", tier: "secondary", github_url: "https://github.com/johnmichealacera/pageant_tabulation_system" },
    ProjectCuration { oid: "69f6ae7307c68ed4285398bf", title: "Socorro 3D View", tier: "secondary", github_url: "https://github.com/johnmichealacera/socorro_3d_view" },
    ProjectCuration { oid: "6821f3e9af25d5ba7e2fe8c1", title: "TeachSlide", tier: "secondary", github_url: "https://github.com/johnmichealacera/teachslide" },
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("personal");
    let collection = db.collection::<mongodb::bson::Document>("projects");

    let mut updated = 0;
    let mut missing = 0;

    for project in PROJECTS {
        let object_id = ObjectId::parse_str(project.oid)?;
        let result = collection
            .update_one(
                doc! { "_id": object_id },
                doc! { "$set": {
                    "tier": project.tier,
                    "githubUrl": project.github_url,
                } },
                None,
            )
            .await?;

        if result.matched_count == 0 {
            eprintln!("No project found for _id {} ({}) — skipped.", project.oid, project.title);
            missing += 1;
        } else {
            println!(
                "Updated \"{}\" -> tier={}, githubUrl={}",
                project.title,
                project.tier,
                if project.github_url.is_empty() { "(none)" } else { project.github_url }
            );
            updated += 1;
        }
    }

    println!("\nDone. {updated} updated, {missing} not found.");
    Ok(())
}
