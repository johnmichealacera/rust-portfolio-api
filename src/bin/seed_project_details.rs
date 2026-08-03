//! Manual, one-off seed script: backfills role/constraints/keyDecisions/impact
//! onto existing `projects` documents by `_id`, matching personal.projects.json.
//! Run with: cargo run --bin seed_project_details
use std::env;

use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client,
};

struct ProjectCaseStudy {
    oid: &'static str,
    title: &'static str,
    role: &'static str,
    constraints: &'static str,
    key_decisions: &'static str,
    impact: &'static str,
}

const PROJECTS: &[ProjectCaseStudy] = &[
    ProjectCaseStudy {
        oid: "63d8b3c2af25d5ba7e2fe8bf",
        title: "HappyChat",
        role: "Designed and built solo end-to-end — conversation flow, AI integration, and deployment.",
        constraints: "No existing product to model a \"positivity-first\" AI chat experience after, so the tone and guardrails had to be defined from scratch.",
        key_decisions: "Kept the interaction deliberately simple — one chat thread, no accounts — so the AI's tone carried the product instead of feature scope.",
        impact: "Live and publicly usable at happy-chat-ten.vercel.app.",
    },
    ProjectCaseStudy {
        oid: "63d8bef70a798e287c123af5",
        title: "Javascript Bird Hunter",
        role: "Built solo in pure vanilla JavaScript, no game engine.",
        constraints: "No physics or rendering library to lean on — collision detection, scoring, and animation timing were all hand-rolled.",
        key_decisions: "Chose vanilla JS on purpose to prove out core game-loop and DOM-rendering fundamentals without framework abstraction in the way.",
        impact: "Live and playable at bird-hunter.johnmichealacera.com.",
    },
    ProjectCaseStudy {
        oid: "645f74b3f4283c07c6213b1d",
        title: "DotaMate",
        role: "Solo-built web app analyzing Dota 2 hero matchups and synergies.",
        constraints: "Matchup and synergy data shifts with every game patch, so the data model had to stay easy to update rather than hardcoded.",
        key_decisions: "Focused the UI on matchup lookups instead of a full stats dashboard, to answer the one question players actually need mid-draft.",
        impact: "Live at dota2-companion.vercel.app.",
    },
    ProjectCaseStudy {
        oid: "645f74dbf4283c07c6213b1e",
        title: "ImageScanner",
        role: "Built the upload-and-detect pipeline end-to-end, integrating a pretrained TensorFlow model into a web UI.",
        constraints: "Inference needed to stay responsive in-browser without a heavy backend service.",
        key_decisions: "Used a pretrained TensorFlow model instead of training a custom one, prioritizing a working end-to-end detection experience over model tuning.",
        impact: "Live at image-scanner.johnmichealacera.com as a working browser-based object-detection demo.",
    },
    ProjectCaseStudy {
        oid: "645f753ff4283c07c6213b1f",
        title: "Taskify",
        role: "Solo-built minimalist todo app in vanilla JavaScript.",
        constraints: "No framework or state-management library — all UI state managed by hand.",
        key_decisions: "Kept the feature set to add/complete/delete only, to focus on clean vanilla-JS state handling over feature breadth.",
        impact: "Live at taskify.johnmichealacera.com.",
    },
    ProjectCaseStudy {
        oid: "66376b9f2ecc465531439667",
        title: "Help App Admin Dashboard",
        role: "Built the admin-dashboard half of a two-part emergency-response system, paired with the CitizenCare mobile app.",
        constraints: "Hospital, fire, and police departments each needed to manage their own announcements and reports without seeing or affecting each other's data.",
        key_decisions: "Built with Next.js around a role-based access model so each department gets its own scoped view instead of one shared, undifferentiated inbox.",
        impact: "Live at help-app-admin.vercel.app, giving emergency departments a working tool to coordinate citizen reports and announcements.",
    },
    ProjectCaseStudy {
        oid: "66376f0ff9ca146bbdf6afc4",
        title: "CitizenCare Mobile App",
        role: "Built the citizen-facing mobile counterpart to the Help App Admin Dashboard.",
        constraints: "Needed to work for everyday citizens with no technical background, so the reporting flow had to be usable on a first try.",
        key_decisions: "Used React Native to ship one codebase across platforms rather than separate native apps, appropriate for a solo/small-scope civic project.",
        impact: "Released as v2.0.0, the citizen-side half of a working two-app system for hospital, fire, and police announcements and reports.",
    },
    ProjectCaseStudy {
        oid: "66e3ae19a3b3b15b4476c806",
        title: "Soroy sa Socorro Website",
        role: "Built solo for a local government unit (Socorro) to showcase tourism and barangay information.",
        constraints: "Content — delicacies, tourist spots, barangay details — needed to stay easy for local government staff to keep current, not a one-time static build.",
        key_decisions: "Prioritized a clear, browsable structure (spots/delicacies/barangays as first-class sections) over a heavier visual redesign, since discoverability mattered more than spectacle for visitors and locals.",
        impact: "Live at soroy-sa-socorro.johnmichealacera.com as the municipality's public tourism and community information site.",
    },
    ProjectCaseStudy {
        oid: "6832d053b6d454632088c95d",
        title: "Skill Match",
        role: "Solo-built marketplace platform connecting blue-collar workers with employers.",
        constraints: "The target users — carpenters, welders, farmers — aren't the typical tech-platform audience, so the interface had to stay simple rather than assume tech familiarity.",
        key_decisions: "Modeled the platform explicitly around blue-collar trades instead of adapting a generic freelance-marketplace template, since Upwork-style products don't fit that audience's workflow.",
        impact: "Live at skill-match-five.vercel.app as a working marketplace for an underserved segment of gig work.",
    },
    ProjectCaseStudy {
        oid: "6857909cf1ddb6fd2c61422a",
        title: "CSS NC2 Companion",
        role: "Built solo as a free educational resource for the Computer Systems Servicing NC II assessment.",
        constraints: "Content needed to map precisely to TESDA's official COC (Certificate of Competency) structure, not a generic tutorial layout.",
        key_decisions: "Organized the site around the official COC breakdown so learners study in the same structure they're assessed in.",
        impact: "Live and free at css-nc2-companion.vercel.app, built to help NC II trainees prepare for a real, formal assessment.",
    },
    ProjectCaseStudy {
        oid: "688ac70b3b332b887d54ae45",
        title: "MD Events and Services",
        role: "Built as a tenant site within the LocalWebVentures multi-tenant platform, covering booking and dashboard control.",
        constraints: "Had to operate within a shared multi-tenant platform's architecture rather than as a fully custom one-off build.",
        key_decisions: "Built the booking and dashboard experience to fit the multi-tenant platform's existing infrastructure, so the client gets independent control without needing separate hosting.",
        impact: "Live at md-events-services.vercel.app as a small business's working booking and service-presentation site.",
    },
    ProjectCaseStudy {
        oid: "69f6ae7307c68ed4285398be",
        title: "Unified Government System",
        role: "Solo-built government workflow platform for disbursement vouchers and approvals.",
        constraints: "Government disbursement involves multi-step approvals with strict accountability — informed directly by earlier hands-on experience inside a government accounting department.",
        key_decisions: "Modeled the system around explicit role-based approval workflows and digital tracking rather than a simple form-and-list CRUD app, so every voucher's approval history stays auditable.",
        impact: "Live at disbursement-voucher-tracker.vercel.app, digitizing a process that previously ran on paper-based approvals.",
    },
    ProjectCaseStudy {
        oid: "69f6ae7307c68ed4285398bd",
        title: "Pageant Tabulation System",
        role: "Solo-built digital tabulation platform for live pageant events.",
        constraints: "Scoring needed to be transparent and defensible in front of a live audience and contestants, not just functionally correct.",
        key_decisions: "Built automated, judge-by-judge score aggregation with visible computations, prioritizing transparency of the ranking process over a purely admin-side scoring tool.",
        impact: "Live at pageant-tabulation-system.vercel.app, used to run real-time scoring and rankings for actual pageant events.",
    },
    ProjectCaseStudy {
        oid: "69f6ae7307c68ed4285398bf",
        title: "Socorro 3D View",
        role: "Solo-built interactive 3D web experience for tourism and presentation.",
        constraints: "Needed to run smoothly in-browser with no install required, so 3D asset weight and rendering performance were constant considerations.",
        key_decisions: "Built as a web-based 3D experience rather than a native app, so it stays instantly accessible to any visitor with a browser.",
        impact: "Live at socorro-3d-view.vercel.app as a public-facing immersive showcase of Socorro.",
    },
    ProjectCaseStudy {
        oid: "6821f3e9af25d5ba7e2fe8c1",
        title: "TeachSlide",
        role: "Solo-built AI-powered presentation generator, drawing directly on personal classroom teaching experience.",
        constraints: "Needed to generate genuinely usable class material — slides, talking points, quizzes — fast enough to actually save prep time, not just produce generic AI output.",
        key_decisions: "Built the generator around the specific artifacts a teacher needs for one class session instead of a general-purpose AI chat wrapper, grounded in first-hand experience running the PF 101/PF 102/WS 101 courses shown in this site's Teaching section.",
        impact: "Live at teachslide.johnmichealacera.com, directly informed by and complementary to the Teaching section of this same site.",
    },
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
                    "role": project.role,
                    "constraints": project.constraints,
                    "keyDecisions": project.key_decisions,
                    "impact": project.impact,
                } },
                None,
            )
            .await?;

        if result.matched_count == 0 {
            eprintln!("No project found for _id {} ({}) — skipped.", project.oid, project.title);
            missing += 1;
        } else {
            println!("Updated \"{}\" ({})", project.title, project.oid);
            updated += 1;
        }
    }

    println!("\nDone. {updated} updated, {missing} not found.");
    Ok(())
}
