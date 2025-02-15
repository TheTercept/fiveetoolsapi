use serde_json::Value;
use std::fs;
use std::path::Path;

const SCHEMA_DIR: &str = "./schema/"; // Path to schema files

/// Load a schema file by name
pub fn load_schema(schema_name: &str) -> Option<Value> {
    let path = Path::new(SCHEMA_DIR).join(schema_name);
    let schema_json = fs::read_to_string(path).ok()?;
    serde_json::from_str(&schema_json).ok()
}

/// Example: Load the monster schema
pub fn get_monster_schema() -> Option<Value> {
    load_schema("monsters.schema.json")
}
