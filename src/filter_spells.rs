use serde_json::Value;
use crate::SpellQuery;

const DEBUG: bool = false;
pub fn filter_spells(data: &Value, query: &SpellQuery) -> Vec<Value> {
    let binding = vec![];
    let all_spells = data.as_array().unwrap_or(&binding);

    all_spells.iter()
        .filter(|spell| {
            let mut match_found = true;

            // Level filter
            if let Some(level_filter) = query.level {
                if let Some(spell_level) = spell.get("level").and_then(|v| v.as_u64()) {
                    if spell_level as u8 != level_filter {
                        match_found = false;
                    }
                }
            }

            //TODO: Filter for class spell

            // Filter by ritual tag
            if let Some(ritual_filter) = query.ritual {
                let is_ritual = spell
                    .get("meta")
                    .and_then(|m| m.get("ritual"))
                    .and_then(|r| r.as_bool())
                    .unwrap_or(false);

                // Ensure only spells that explicitly have "ritual": true are included
                if ritual_filter && !is_ritual {
                    match_found = false;
                }

                // If filtering for non-rituals (false), exclude all spells that have "ritual": true
                if !ritual_filter && is_ritual {
                    match_found = false;
                }
            }


            // Filter by spell school
            if let Some(ref school_filter) = query.school {
                if let Some(spell_school) = spell.get("school").and_then(|v| v.as_str()) {
                    if spell_school.to_lowercase() != school_filter.to_lowercase() {
                        match_found = false;
                    }
                }
            }

            // Filter by casting time category
            if let Some(ref time_filter) = query.casting_time {
                if let Some(times) = spell.get("time").and_then(|v| v.as_array()) {
                    let found = times.iter().any(|t| {
                        let num = t.get("number").and_then(|n| n.as_u64()).unwrap_or(0);
                        let unit = t.get("unit").and_then(|u| u.as_str()).unwrap_or("");
                        let is_ritual = t.get("condition").and_then(|c| c.as_str()) == Some("ritual");

                        // Map time values into categories
                        let spell_casting_category = match (num, unit) {
                            (1, "action") => "action",
                            (1, "bonus") => "bonus action", // Fix: Ensure bonus actions are separate
                            (1, "reaction") => "reaction",
                            (1, "minute") => "1 minute",
                            (10, "minute") => "10 minute",
                            (1, "hour") => "1 hour",
                            (_, "hour") if num > 1 => "more than one hour",
                            _ => "other",
                        };

                        spell_casting_category == time_filter
                    });

                    if !found {
                        match_found = false;
                    }
                }
            }

            // Filter by range
            if let Some(ref range_filter) = query.range {
                if let Some(range_value) = spell.get("range").and_then(|r| r["type"].as_str()) {
                    if range_value.to_lowercase() != range_filter.to_lowercase() {
                        match_found = false;
                    }
                }
            }

            // Filter by spell components (V, S, M)
            if let Some(component_v) = query.component_v {
                let has_verbal = spell
                    .get("components")
                    .and_then(|c| c.get("v"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if component_v && !has_verbal {
                    match_found = false;
                }
                if !component_v && has_verbal {
                    match_found = false;
                }
            }

            if let Some(component_s) = query.component_s {
                let has_somatic = spell
                    .get("components")
                    .and_then(|c| c.get("s"))
                    .and_then(|s| s.as_bool())
                    .unwrap_or(false);

                if component_s && !has_somatic {
                    match_found = false;
                }
                if !component_s && has_somatic {
                    match_found = false;
                }
            }

            if let Some(component_m) = query.component_m {
                let has_material = spell
                    .get("components")
                    .and_then(|c| c.get("m"))
                    .map(|m| match m {
                        Value::Bool(b) => *b,           // If it's a boolean, use its value
                        Value::Object(_) => true,       // If it's an object (has material details), treat as true
                        _ => false,                     // Otherwise, false
                    })
                    .unwrap_or(false);

                if component_m && !has_material {
                    match_found = false;
                }
                if !component_m && has_material {
                    match_found = false;
                }
            }

            // Filter by duration type
            if let Some(ref duration_filter) = query.duration {
                if let Some(durations) = spell.get("duration").and_then(|v| v.as_array()) {
                    let found = durations.iter().any(|d| {
                        let duration_type = d.get("type").and_then(|t| t.as_str()).unwrap_or("");
                        duration_type == duration_filter
                    });

                    if !found {
                        match_found = false;
                    }
                }
            }

            // Filter by concentration
            if let Some(concentration_filter) = query.concentration {
                let is_concentration = spell
                    .get("duration")
                    .and_then(|d| d.as_array())
                    .map(|durations| durations.iter().any(|d| {
                        d.get("concentration").and_then(|c| c.as_bool()).unwrap_or(false)
                    }))
                    .unwrap_or(false);

                if is_concentration != concentration_filter {
                    match_found = false;
                }
            }


            // Print whether spell passed filters
            if DEBUG {
                if match_found {
                    println!("✅ Spell Passed Filters: {:?}", spell.get("name"));
                } else {
                    println!("❌ Spell Filtered Out: {:?}", spell.get("name"));
                }
            }

            match_found
        })
        .cloned()
        .collect()
}