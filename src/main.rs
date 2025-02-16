use axum::{routing::get, Router, extract::Query, response::Json};
use serde_json::Value;
use std::fs;
use std::net::SocketAddr;
use serde::Deserialize;

mod schema;
mod filter_monsters;
mod filter_spells;

/// Load user data from the file or return an empty JSON array if there's an error
fn load_user_data(path: &str, key: &str) -> Value {
    let content = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    let data: Value = serde_json::from_str(&content).unwrap_or_else(|_| {
        println!("Failed to parse JSON from {}", path);
        serde_json::json!({ key: [] })
    });

    let extracted_data = data.get(key).cloned().unwrap_or_else(|| {
        println!("Key '{}' not found in {}", key, path);
        serde_json::json!([])
    });

    //println!("Loaded data for '{}': {:?}", key, extracted_data);
    extracted_data
}


/// Define query parameters structure
#[derive(Deserialize)]
struct MonsterQuery {
    type_: Option<String>,
    cr: Option<String>,
    size: Option<String>,
    alignment: Option<String>,
    ac: Option<String>,
    hp: Option<String>,
    speed: Option<String>,
    environment: Option<String>,
    speed_type: Option<String>
}

#[derive(Deserialize)]
struct SpellQuery {
    level: Option<u8>,
    school: Option<String>,
    casting_time: Option<String>,
    range: Option<String>,
    component_v: Option<bool>,
    component_s: Option<bool>,
    component_m: Option<bool>,
    duration: Option<String>,
    concentration: Option<bool>,
    ritual: Option<bool>
}

/// Endpoint to query monsters with filtering based on the query parameters
async fn query_monsters(query: Query<MonsterQuery>) -> Json<Value> {
    let schema = schema::get_monster_schema().unwrap();  // Load monster schema
    let filters = filter_monsters::generate_filters(&schema);    // Generate filters from schema

    // Load user-provided monster data
    let data = load_user_data("./user_data/bestiary-mm.json", "monster");
    // Apply the filters to the loaded data
    let filtered_monsters = filter_monsters::filter_monsters(&data, &query);

    // Return the filtered monsters in JSON format
    Json(serde_json::json!({ "monsters": filtered_monsters }))
}

async fn query_spells(query: Query<SpellQuery>) -> Json<Value> {
    let data = load_user_data("./user_data/spells-phb.json", "spell");

    let filtered_spells = filter_spells::filter_spells(&data, &query);
    Json(serde_json::json!({ "spells": filtered_spells }))
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Json(serde_json::json!({ "message": "Local 5eTools API" })) }))
        .route("/monsters", get(query_monsters))
        .route("/spells", get(query_spells));

    // Start the server on port 8000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}