use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

static STORE_PATH: &str = "settings.json";
const KEY_MIN_INTERVAL: &str = "min_interval";
const KEY_MAX_INTERVAL: &str = "max_interval";
const KEY_START_ENABLED: &str = "start_enabled";

pub fn initialize_store(app: &AppHandle) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;

    if !store.has(KEY_MIN_INTERVAL) {
        store.set(KEY_MIN_INTERVAL, json!(1u64));
    }
    if !store.has(KEY_MAX_INTERVAL) {
        store.set(KEY_MAX_INTERVAL, json!(30u64));
    }
    if !store.has(KEY_START_ENABLED) {
        store.set(KEY_START_ENABLED, json!(false));
    }

    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_min_interval(app: &AppHandle) -> u64 {
    app.store(STORE_PATH)
        .ok()
        .and_then(|store| store.get(KEY_MIN_INTERVAL))
        .and_then(|v| v.as_u64())
        .unwrap_or(1)
}

pub fn get_max_interval(app: &AppHandle) -> u64 {
    app.store(STORE_PATH)
        .ok()
        .and_then(|store| store.get(KEY_MAX_INTERVAL))
        .and_then(|v| v.as_u64())
        .unwrap_or(30)
}

pub fn set_intervals(app: &AppHandle, min: u64, max: u64) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;

    store.set(KEY_MIN_INTERVAL, json!(min));
    store.set(KEY_MAX_INTERVAL, json!(max));

    store.save().map_err(|e| e.to_string())
}

pub fn get_start_enabled(app: &AppHandle) -> bool {
    app.store(STORE_PATH)
        .ok()
        .and_then(|store| store.get(KEY_START_ENABLED))
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

pub fn set_start_enabled(app: &AppHandle, enabled: bool) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    store.set(KEY_START_ENABLED, json!(enabled));
    store.save().map_err(|e| e.to_string())
}
