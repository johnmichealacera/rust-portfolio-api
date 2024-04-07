use axum::{
    routing::{get, post}, Extension, Router
};
use std::error::Error as StdError;
use mongodb::{bson::{self}, error::Error, options::ClientOptions, Client, Collection, Database};
use dotenv::dotenv;
use serde_json::Value;
use tokio::net::TcpListener;
use std::{
    env, sync::{Arc, Mutex}
};
use serde::{Deserialize, Serialize};
use juniper::{
    graphql_object, graphql_value, EmptyMutation, EmptySubscription, FieldError, RootNode
};
use juniper_axum::graphql;
use mongodb::bson::Document;

#[derive(Clone, Copy, Debug, Default)]
pub struct Context;

impl juniper::Context for Context {}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[derive(Debug, Deserialize, Serialize, juniper::GraphQLObject)]
struct Introduction {
    title: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize, juniper::GraphQLObject)]
struct Personal {
    email: String,
    #[serde(rename = "jobDescription")]
    job_description: String,
    #[serde(rename = "lifeStory")]
    life_story: String,
    #[serde(rename = "whyDothis")]
    why_do_this: String,
    #[serde(rename = "backgroundUrl")]
    background_url: String,
}

#[derive(Debug, Deserialize, Serialize, juniper::GraphQLObject)]
struct Project {
    email: String,
    title: String,
    description: String,
    url: String,
    #[serde(rename = "backgroundImage")]
    background_image: String,
}

#[derive(Debug, Deserialize, Serialize, juniper::GraphQLObject)]
struct SkillsOverview {
    email: String,
    title: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize, juniper::GraphQLObject)]
struct Skills {
    name: String,
    mastery: i32,
    #[serde(rename = "skillType")]
    skill_type: String,
}

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    // Resolver function to fetch introductions
    async fn introductions() -> Result<Vec<Introduction>, FieldError> {
        match get_data_db(String::from("introductions")).await {
            Ok(values) => {
                let introductions: Vec<Introduction> = values
                    .into_iter()
                    .filter_map(|value| value_to_introduction(value).ok())
                    .collect();
                Ok(introductions)
            }
            Err(err) => Err(FieldError::new(
                "Failed to fetch introductions",
                graphql_value!({ "details": err.to_string() }),
            )),
        }
    }
    // Resolver function to fetch personals
    async fn personals() -> Result<Vec<Personal>, FieldError> {
        match get_data_db(String::from("personals")).await {
            Ok(values) => {
                let personals: Vec<Personal> = values
                    .into_iter()
                    .filter_map(|value| value_to_personal(value).ok())
                    .collect();
                Ok(personals)
            }
            Err(err) => Err(FieldError::new(
                "Failed to fetch personals",
                graphql_value!({ "details": err.to_string() }),
            )),
        }
    }
    // Resolver function to fetch projects
    async fn projects() -> Result<Vec<Project>, FieldError> {
        match get_data_db(String::from("projects")).await {
            Ok(values) => {
                let projects: Vec<Project> = values
                    .into_iter()
                    .filter_map(|value| value_to_project(value).ok())
                    .collect();
                Ok(projects)
            }
            Err(err) => Err(FieldError::new(
                "Failed to fetch personals",
                graphql_value!({ "details": err.to_string() }),
            )),
        }
    }
    // Resolver function to fetch skills overview
    async fn skills_overview() -> Result<Vec<SkillsOverview>, FieldError> {
        match get_data_db(String::from("skillsoverview")).await {
            Ok(values) => {
                let skills_overview: Vec<SkillsOverview> = values
                    .into_iter()
                    .filter_map(|value| value_to_skillsoverview(value).ok())
                    .collect();
                Ok(skills_overview)
            }
            Err(err) => Err(FieldError::new(
                "Failed to fetch skills overview",
                graphql_value!({ "details": err.to_string() }),
            )),
        }
    }
    // Resolver function to fetch skills
    async fn skills() -> Result<Vec<Skills>, FieldError> {
        match get_data_db(String::from("skills")).await {
            Ok(values) => {
                let skills: Vec<Skills> = values
                    .into_iter()
                    .filter_map(|value| value_to_skills(value).ok())
                    .collect();
                Ok(skills)
            }
            Err(err) => Err(FieldError::new(
                "Failed to fetch skills",
                graphql_value!({ "details": err.to_string() }),
            )),
        }
    }
}


#[tokio::main]
async fn main() {
    // Load the .env file
    dotenv().ok();
    let schema = Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new()
     );
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        // .route("/introduction", post(create_introduction))
        // .route("/:collection_name", get(get_handler))
        .route("/graphql", post(graphql::<Arc<Schema>>))
        .layer(Extension(Arc::new(schema)));
    let axum_address = env::var("AXUM_ADDRESS").expect("AXUM_ADDRESS must be set");
    let app_port = env::var("PORT").expect("PORT must be set");
    let axum_listener_address = format!("{}:{}", axum_address, app_port);
    let listener = TcpListener::bind(&axum_listener_address).await.expect("Failed to bind to address");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, JM AAcera man!"
}

async fn connect_to_database() -> Result<Database, mongodb::error::Error> {
    // Create a new MongoConnection instance
    let connection_result = MongoConnection::new().await;
    match connection_result {
        Ok(connection) => {
            // Connection successful
            println!("Connected to MongoDB");
            // Example usage: Get a handle to a database
            let db = connection.db("personal");
            Ok(db)
        }
        Err(e) => {
            // Handle connection error
            eprintln!("Error connecting to MongoDB: {}", e);
            Err(e)
        }
    }
}

async fn find_all(db: &Database, collection_name: &str) -> Result<Vec<Value>, Error> {
    let collection: Collection<Document> = db.collection(collection_name);
    // Construct the filter document to match the email field
    let filter = bson::doc! { "email": "acerajohnmicheal@gmail.com" };
    let mut cursor = collection.find(filter, None).await?;
    let mut documents = Vec::new();

    while let true = cursor.advance().await? {
        let doc = cursor.deserialize_current();
        match doc {
            Ok(document) => {
                // Here, document is of type Document
                // Convert the BSON Document into a serde_json::Value
                let doc_as_json = bson::Bson::Document(document).into();
                
                documents.push(doc_as_json);
            },
            Err(e) => eprintln!("Error deserializing document: {}", e),
        }
    }
    Ok(documents)
}

fn value_to_personal(value: Value) -> Result<Personal, Box<dyn StdError>> {
    match serde_json::from_value(value) {
        Ok(personal) => Ok(personal),
        Err(e) => Err(e.into()),
    }
}

fn value_to_project(value: Value) -> Result<Project, Box<dyn StdError>> {
    match serde_json::from_value(value) {
        Ok(project) => Ok(project),
        Err(e) => Err(e.into()),
    }
}

fn value_to_skillsoverview(value: Value) -> Result<SkillsOverview, Box<dyn StdError>> {
    match serde_json::from_value(value) {
        Ok(skills_overview) => Ok(skills_overview),
        Err(e) => Err(e.into()),
    }
}

fn value_to_skills(value: Value) -> Result<Skills, Box<dyn StdError>> {
    match serde_json::from_value(value) {
        Ok(skills) => Ok(skills),
        Err(e) => Err(e.into()),
    }
}

fn value_to_introduction(value: Value) -> Result<Introduction, Box<dyn StdError>> {
    match serde_json::from_value(value) {
        Ok(introduction) => Ok(introduction),
        Err(e) => Err(e.into()),
    }
}

async fn get_data_db(collection_name: String) -> Result<Vec<Value>, Error> {
    // Connect to the database
    let database = connect_to_database().await?;

    // Fetch all documents from the "personals" collection
    let values = find_all(&database, collection_name.as_str()).await?;
    Ok(values)
}


pub struct MongoConnection {
    client: Arc<Mutex<Client>>,
}

impl MongoConnection {
    pub async fn new() -> Result<Self, Error> {
        let mongo_db_uri = env::var("MONGO_DB_URI")
            .unwrap_or_else(|_| {
                println!("MONGO_DB_URI is not set, using default value");
                "default_value".to_string()
            });
        let client_options = ClientOptions::parse(mongo_db_uri).await?;
        let client = Client::with_options(client_options)?;

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub fn db(&self, name: &str) -> Database {
        self.client.lock().unwrap().database(name)
    }
}