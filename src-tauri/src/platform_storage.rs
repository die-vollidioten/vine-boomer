use std::error::Error;

const APP_NAME: &str = "vine-boomer";
const KEY_NAME: &str = "installed";

#[cfg(target_os = "windows")]
pub fn save_bool(value: bool) -> Result<(), Box<dyn Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(&format!("SOFTWARE\\{}", APP_NAME))?;
    key.set_value(KEY_NAME, &(value as u32))?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn save_bool(value: bool) -> Result<(), Box<dyn Error>> {
    use keyring::Entry;
    let entry = Entry::new(APP_NAME, KEY_NAME)?;
    entry.set_password(&value.to_string())?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn load_bool() -> Result<bool, Box<dyn Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey(&format!("SOFTWARE\\{}", APP_NAME))?;
    let value: u32 = key.get_value(KEY_NAME)?;
    Ok(value != 0)
}

#[cfg(not(target_os = "windows"))]
pub fn load_bool() -> Result<bool, Box<dyn Error>> {
    use keyring::Entry;
    let entry = Entry::new(APP_NAME, KEY_NAME)?;
    let value = entry.get_password()?;
    Ok(value == "true")
}