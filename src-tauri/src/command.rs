use crate::{storage, VINE_BOOM_ENABLED, GENERATION};
use std::sync::atomic::Ordering;
use tauri::AppHandle;

#[derive(serde::Serialize)]
pub struct Status {
    enabled: bool,
    min_time: u64,
    max_time: u64,
}

#[tauri::command]
pub fn get_status(app: AppHandle) -> Status {
    Status {
        enabled: VINE_BOOM_ENABLED.load(Ordering::Relaxed),
        min_time: storage::get_min_interval(&app),
        max_time: storage::get_max_interval(&app),
    }
}

#[tauri::command]
pub fn toggle_status() -> bool {
    let current = VINE_BOOM_ENABLED.load(Ordering::Relaxed);
    let new_status = !current;
    VINE_BOOM_ENABLED.store(new_status, Ordering::Relaxed);
    new_status
}

#[tauri::command]
pub fn set_interval(app: AppHandle, min: u64, max: u64) -> Result<(), String> {
    if min > max {
        return Err("Minimum time cannot be greater than maximum time".to_string());
    }
    GENERATION.fetch_add(1, Ordering::Relaxed);
    storage::set_intervals(&app, min, max)
}

#[tauri::command]
pub fn is_autostart_enabled(app_handle: AppHandle) -> bool {
    crate::is_autostart_enabled(&app_handle)
}

#[tauri::command]
pub fn enable_autostart(app_handle: AppHandle) -> Result<(), String> {
    crate::enable_autostart(&app_handle)
}

#[tauri::command]
pub fn disable_autostart(app_handle: AppHandle) -> Result<(), String> {
    crate::disable_autostart(&app_handle)
}
