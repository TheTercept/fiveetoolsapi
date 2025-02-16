use serde_json::Value;
use crate::SpellQuery;

pub fn filter_spells(data: &Value, query: &SpellQuery) -> Vec<Value> {
    data.get("spell")
        .and_then(|spells| spells.as_array())
        .unwrap_or(&vec![])
        .iter()
        .filter(|spell| {
            let mut match_found = true;

            //TODO: Filter for class spell
            //TODO: Filter for ritual casting
            //TODO: Fix spell array listing

            // Filter by spell level
            if let Some(level_filter) = query.level {
                if let Some(spell_level) = spell.get("level").and_then(|v| v.as_u64()) {
                    if spell_level as u8 != level_filter {
                        match_found = false;
                    }
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

            // Filter by casting time
            //TODO: Switch casting time from a number to: action, bonus action, reaction, 1 minute, 1 hour, more than one hour
            if let Some(ref time_filter) = query.casting_time {
                if let Some(times) = spell.get("time").and_then(|v| v.as_array()) {
                    let found = times.iter().any(|t| {
                        t.get("number")
                            .and_then(|n| n.as_u64())
                            .map(|num| num.to_string())
                            .unwrap_or_default()
                            == *time_filter
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

            // Filter by components
            if let Some(component_v) = query.component_v {
                if let Some(v) = spell.get("components").and_then(|c| c["v"].as_bool()) {
                    if v != component_v {
                        match_found = false;
                    }
                }
            }

            if let Some(component_s) = query.component_s {
                if let Some(s) = spell.get("components").and_then(|c| c["s"].as_bool()) {
                    if s != component_s {
                        match_found = false;
                    }
                }
            }

            if let Some(component_m) = query.component_m {
                if let Some(m) = spell.get("components").and_then(|c| c["m"].as_bool()) {
                    if m != component_m {
                        match_found = false;
                    }
                }
            }

            // Filter by duration
            if let Some(ref duration_filter) = query.duration {
                if let Some(duration_value) = spell.get("duration").and_then(|d| d.as_str()) {
                    if !duration_value.to_lowercase().contains(&duration_filter.to_lowercase()) {
                        match_found = false;
                    }
                }
            }

            match_found
        })
        .cloned()
        .collect()
}
