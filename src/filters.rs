use std::collections::{HashSet};
use serde_json::Value;
use crate::MonsterQuery;

pub fn generate_filters(schema: &Value) -> HashSet<String> {
    let mut filters = HashSet::new();

    if let Some(properties) = schema["properties"].as_object() {
        for (key, value) in properties {
            if let Some(field_type) = value["type"].as_str() {
                // Only allow string, number, or boolean fields as filters
                if ["string", "number", "boolean"].contains(&field_type) {
                    filters.insert(key.clone());
                }
            }
        }
    }
    filters
}

pub fn filter_monsters(data: &Value, filters: &HashSet<String>, query: &MonsterQuery) -> Vec<Value> {
    data.as_array()
        .unwrap_or(&vec![])  // Handle case where data is not an array
        .iter()
        .filter(|monster| {
            let mut match_found = true;

            // Check each query parameter and filter the monsters accordingly
            if let Some(ref type_filter) = query.type_ {
                if let Some(type_value) = monster.get("type") {
                    let monster_type = match type_value {
                        Value::String(s) => s.clone(),  // Simple case: "type": "Aberration"
                        Value::Object(obj) => obj.get("type").and_then(|t| t.as_str()).unwrap_or("").to_string(),
                        _ => "".to_string(),
                    };
                    println!("Filtering by type: Expected '{}', Found '{}'", type_filter, monster_type); // Debugging
                    if monster_type != *type_filter {
                        match_found = false;
                    }
                }
            }

            if let Some(ref cr_filter) = query.cr {
                if let Some(cr_value) = monster.get("cr") {
                    let monster_cr = match cr_value {
                        Value::String(s) => s.clone(),  // Simple case: "cr": "1"
                        Value::Number(n) => n.to_string(), // If stored as a number
                        _ => "".to_string(),
                    };
                    println!("Filtering by CR: Expected '{}', Found '{}'", cr_filter, monster_cr); // Debugging
                    if monster_cr != *cr_filter {
                        match_found = false;
                    }
                }
            }

            match_found
        })
        .cloned()
        .collect()  // Return a vector of filtered monsters
}

