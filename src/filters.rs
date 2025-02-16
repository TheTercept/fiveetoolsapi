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
        .unwrap_or(&vec![])
        .iter()
        .filter(|monster| {
            let mut match_found = true;

            // Type filter
            if let Some(ref type_filter) = query.type_ {
                if let Some(type_value) = monster.get("type") {
                    let monster_type = match type_value {
                        Value::String(s) => s.clone(),
                        Value::Object(obj) => obj.get("type").and_then(|t| t.as_str()).unwrap_or("").to_string(),
                        _ => "".to_string(),
                    };
                    if monster_type != *type_filter {
                        match_found = false;
                    }
                }
            }

            // Challenge Rating (CR) filter
            if let Some(ref cr_filter) = query.cr {
                if let Some(cr_value) = monster.get("cr") {
                    let monster_cr = match cr_value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => "".to_string(),
                    };
                    if monster_cr != *cr_filter {
                        match_found = false;
                    }
                }
            }

            // Size filter
            if let Some(ref size_filter) = query.size {
                let size_char = match size_filter.as_str() {
                    "Tiny" => "T",
                    "Small" => "S",
                    "Medium" => "M",
                    "Large" => "L",
                    "Huge" => "H",
                    "Gargantuan" => "G",
                    _ => "",
                };

                if let Some(size_value) = monster.get("size").and_then(|v| v.as_array()) {
                    let monster_size = size_value.get(0).and_then(|v| v.as_str()).unwrap_or("");
                    if monster_size != size_char {
                        match_found = false;
                    }
                }
            }

            // Alignment filter
            if let Some(ref alignment_filter) = query.alignment {
                let alignment_chars: Vec<&str> = match alignment_filter.as_str() {
                    "Chaotic Good" => vec!["C", "G"],
                    "Chaotic Neutral" => vec!["C", "N"],
                    "Chaotic Evil" => vec!["C", "E"],
                    "Neutral Good" => vec!["N", "G"],
                    "Neutral" => vec!["N"],
                    "Neutral Evil" => vec!["N", "E"],
                    "Lawful Good" => vec!["L", "G"],
                    "Lawful Neutral" => vec!["L", "N"],
                    "Lawful Evil" => vec!["L", "E"],
                    _ => vec![],
                };

                if let Some(alignment_values) = monster.get("alignment").and_then(|v| v.as_array()) {
                    let monster_alignment: Vec<&str> = alignment_values.iter().filter_map(|v| v.as_str()).collect();
                    if !alignment_chars.iter().all(|char| monster_alignment.contains(char)) {
                        match_found = false;
                    }
                }
            }

            // Armor Class (AC) filter
            if let Some(ref ac_filter) = query.ac {
                if let Some(ac_value) = monster.get("ac") {
                    let monster_ac = match ac_value {
                        Value::Number(n) => n.to_string(),
                        Value::Array(arr) => arr.get(0).and_then(|v| v["ac"].as_i64()).map(|v| v.to_string()).unwrap_or("".to_string()),
                        _ => "".to_string(),
                    };
                    if monster_ac != *ac_filter {
                        match_found = false;
                    }
                }
            }

            // Hit Points (HP) filter
            if let Some(ref hp_filter) = query.hp {
                if let Some(hp_value) = monster.get("hp") {
                    let monster_hp = match hp_value {
                        Value::Number(n) => n.to_string(),
                        Value::Object(obj) => obj.get("average").and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or("".to_string()),
                        _ => "".to_string(),
                    };
                    if monster_hp != *hp_filter {
                        match_found = false;
                    }
                }
            }

            // Speed filter
            if let Some(ref speed_filter) = query.speed {
                let speed_type = query.speed_type.as_deref().unwrap_or("walk"); // Default to "walk" if not specified

                if let Some(speed_value) = monster.get("speed") {
                    let monster_speed = match speed_value {
                        Value::Object(obj) => obj.get(speed_type).and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or("".to_string()),
                        Value::String(s) => s.clone(),  // Some monsters store speed as a single string
                        _ => "".to_string(),
                    };

                    if monster_speed != *speed_filter {
                        match_found = false;
                    }
                }
            }


            // Environment filter (allowing partial matches & case-insensitive search)
            // NOTE: Not all creatures have a listed environment, those with no environment will be shown with this filter
            if let Some(ref env_filter) = query.environment {
                if let Some(env_value) = monster.get("environment") {
                    let monster_env = match env_value {
                        Value::Array(arr) => arr.iter().any(|v| {
                            v.as_str()
                                .map(|s| s.to_lowercase().contains(&env_filter.to_lowercase()))
                                .unwrap_or(false)
                        }),
                        Value::String(s) => s.to_lowercase().contains(&env_filter.to_lowercase()),
                        _ => false,
                    };

                    if !monster_env {
                        match_found = false;
                    }
                }
            }

            match_found
        })
        .cloned()
        .collect()
}