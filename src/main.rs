use axum::{routing::get, Router, extract::Query, response::Json};
use serde_json::Value;
use std::fs;
use std::net::SocketAddr;
use serde::Deserialize;

mod schema;
mod filters;

/// Load user data from the file or return an empty JSON array if there's an error
fn load_user_data(path: &str) -> Value {
    let content = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    let data: Value = serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({ "monster": [] }));

    // Ensure we're returning the actual monster array
    data.get("monster").cloned().unwrap_or_else(|| serde_json::json!([]))
}

/// Define query parameters structure
#[derive(Deserialize)]
struct MonsterQuery {
    type_: Option<String>,  // `type_` instead of `type` because `type` is a reserved keyword
    cr: Option<String>,     // Challenge Rating filter
}

/// Endpoint to query monsters with filtering based on the query parameters
async fn query_monsters(query: Query<MonsterQuery>) -> Json<Value> {
    let schema = schema::get_monster_schema().unwrap();  // Load monster schema
    let filters = filters::generate_filters(&schema);    // Generate filters from schema

    // Load user-provided monster data
    let data = load_user_data("./user_data/bestiary-mm.json");

    // Apply the filters to the loaded data
    let filtered_monsters = filters::filter_monsters(&data, &filters, &query);

    // Return the filtered monsters in JSON format
    Json(serde_json::json!({ "monsters": filtered_monsters }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Json(serde_json::json!({ "message": "Local 5eTools API" })) }))
        .route("/monsters", get(query_monsters));

    // Start the server on port 8000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}