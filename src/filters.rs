use serde_json::Value;
use std::collections::HashSet;

/// Extract possible filters from a schema
pub fn generate_filters(schema: &Value) -> HashSet<String> {
    let mut filters = HashSet::new();

    if let Some(properties) = schema["items"]["properties"].as_object() {
        for (key, value) in properties {
            // Only allow string, number, or boolean fields as filters
            if let Some(field_type) = value["type"].as_str() {
                if ["string", "number", "boolean"].contains(&field_type) {
                    filters.insert(key.clone());
                }
            }
        }
    }
    filters
}
