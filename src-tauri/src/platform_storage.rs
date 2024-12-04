#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(not(target_os = "windows"))]
use keyring::Entry;


pub fn save_bool(value: bool) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let app_key = hkcu.create_subkey("Software\\MyApp")?;
        app_key.set_value("VineInstalled", &value)?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let entry = Entry::new("my_app", "my_bool");
        entry.set_password(&value.to_string())?;
    }

    Ok(())
}


pub fn load_bool() -> Result<bool, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let app_key = hkcu.open_subkey("Software\\MyApp")?;
        let value: bool = app_key.get_value("VineInstalled")?;
        return Ok(value);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let entry = Entry::new("my_app", "VineInstalled");
        let password = entry.get_password()?;
        return Ok(password == "true");
    }
}