mod schema;
mod filters;
use axum::{routing::get, Router, extract::Query, response::Json};
use serde_json::Value;
use std::fs;
use std::collections::HashMap;
use std::net::SocketAddr;

// Load user data or return an empty array if there is an error
fn load_user_data(path: &str) -> Value {
    let content = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!([]))
}

// Filter user data dynamically
async fn query_monsters(query: Query<HashMap<String, String>>) -> Json<Value> {
    let schema = schema::get_monster_schema().unwrap();
    let filters = filters::generate_filters(&schema);

    let data = load_user_data("/root/code/fiveetoolsapi/user_data/bestiary-mm.json");
	println!("Loaded Data: {:?}", data);  // Debug: print the loaded data

    // Create an empty vector (owned) for cases where data is not an array
    let empty_vec: Vec<Value> = Vec::new();
    let monsters = if let Some(arr) = data.as_array() {
        arr
    } else {
        &empty_vec  // Now use the owned empty vector
    };

    // Apply filters
    let filtered_monsters: Vec<&Value> = monsters.iter()
        .filter(|m| {
            for (key, value) in query.iter() {
                if !filters.contains(key) { continue; }
                if m[key] != Value::String(value.clone()) {
                    return false;
                }
            }
            true
        })
        .collect();

    Json(serde_json::json!({ "monsters": filtered_monsters }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Json(serde_json::json!({ "message": "Local 5eTools API" })) }))
        .route("/monsters", get(query_monsters));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
