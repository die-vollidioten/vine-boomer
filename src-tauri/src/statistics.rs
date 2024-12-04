use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDateTime};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoomEvent {
    timestamp: i64,
    was_rare: bool,
    sound_path: String,
    time_since_last: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BoomStats {
    total_booms: u64,
    rare_booms: u64,
    daily_booms: HashMap<String, u64>,
    average_interval: f64,
    last_boom: Option<i64>,
    events: Vec<BoomEvent>,
}

const STATS_PATH: &str = "boom_stats.json";
const STATS_KEY: &str = "stats";

pub fn record_boom(app: &AppHandle, was_rare: bool, sound_path: String) {
    if let Ok(store) = app.store(STATS_PATH) {
        let mut stats: BoomStats = store
            .get(STATS_KEY)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let now = Utc::now();
        let timestamp = now.timestamp();
        let today = now.date_naive().to_string();

        // Update basic stats
        stats.total_booms += 1;
        if was_rare {
            stats.rare_booms += 1;
        }

        // Update daily stats
        *stats.daily_booms.entry(today).or_insert(0) += 1;

        // Calculate time since last boom
        let time_since_last = stats.last_boom.map(|last| {
            (timestamp - last) as u64
        });

        // Update average interval
        if let Some(interval) = time_since_last {
            if stats.average_interval == 0.0 {
                stats.average_interval = interval as f64;
            } else {
                stats.average_interval = (stats.average_interval * 0.9) + (interval as f64 * 0.1);
            }
        }

        // Record the event
        stats.events.push(BoomEvent {
            timestamp,
            was_rare,
            sound_path,
            time_since_last,
        });

        stats.last_boom = Some(timestamp);

        // Save stats
        let _ = store.set(STATS_KEY, json!(stats));
        let _ = store.save();
    }
}

pub fn get_stats(app: &AppHandle) -> BoomStats {
    app.store(STATS_PATH)
        .ok()
        .and_then(|store| store.get(STATS_KEY))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default()
} 