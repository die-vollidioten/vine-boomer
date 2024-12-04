use std::error::Error;
use crate::platform_storage;
use log::{info, error};

pub async fn check_and_track_install() -> Result<(), Box<dyn Error>> {
    let is_installed = platform_storage::load_bool().unwrap_or(false);
    
    if !is_installed {
        match register_with_server().await {
            Ok(_) => {
                platform_storage::save_bool(true)?;
                info!("Successfully registered with the server.");
            }
            Err(e) => {
                error!("Failed to register with the server: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn register_with_server() -> Result<(), Box<dyn Error>> {
    let success = true;

    if success {
        Ok(())
    } else {
        Err("Server registration failed".into())
    }
} 