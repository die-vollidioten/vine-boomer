use crate::{MAX_INTERVAL, MIN_INTERVAL, VINE_BOOM_ENABLED};
use std::sync::atomic::Ordering;

#[derive(serde::Serialize)]
pub struct Status {
    enabled: bool,
    min_time: u64,
    max_time: u64,
}

#[tauri::command]
pub fn get_status() -> Status {
    Status {
        enabled: VINE_BOOM_ENABLED.load(Ordering::Relaxed),
        min_time: MIN_INTERVAL.load(Ordering::Relaxed),
        max_time: MAX_INTERVAL.load(Ordering::Relaxed),
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
pub fn set_interval(min: u64, max: u64) -> Result<(), String> {
    if min > max {
        return Err("Minimum time cannot be greater than maximum time".to_string());
    }

    MIN_INTERVAL.store(min, Ordering::Relaxed);
    MAX_INTERVAL.store(max, Ordering::Relaxed);
    Ok(())
}
