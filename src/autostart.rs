//! Running the watcher automatically when the user logs in.

/// The name of our entry among the programs that run at login.
#[cfg_attr(not(target_os = "windows"), allow(unused))]
const ENTRY: &str = "Ludusavi";

/// The command that should run at login.
#[cfg_attr(not(target_os = "windows"), allow(unused))]
fn command() -> Option<String> {
    let executable = std::env::current_exe().ok()?;
    Some(format!("\"{}\" watch --background", executable.display()))
}

/// Whether Ludusavi is set to watch for games at login.
pub fn enabled() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::{HKEY_CURRENT_USER, KEY_READ};

        let Ok(key) = winreg::RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags(r"Software\Microsoft\Windows\CurrentVersion\Run", KEY_READ)
        else {
            return false;
        };

        key.get_value::<String, _>(ENTRY).is_ok()
    }

    #[cfg(not(target_os = "windows"))]
    false
}

/// Start or stop watching for games at login.
#[cfg_attr(not(target_os = "windows"), allow(unused))]
pub fn set_enabled(enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};

        let key = winreg::RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags(r"Software\Microsoft\Windows\CurrentVersion\Run", KEY_SET_VALUE)
            .map_err(|e| e.to_string())?;

        if enabled {
            let command = command().ok_or_else(|| "Unable to find the Ludusavi executable".to_string())?;
            key.set_value(ENTRY, &command).map_err(|e| e.to_string())
        } else {
            match key.delete_value(ENTRY) {
                Ok(_) => Ok(()),
                // It's fine if there was nothing to remove.
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    Err("Not supported on this operating system".to_string())
}

/// Whether this operating system supports running Ludusavi at login.
pub fn supported() -> bool {
    cfg!(target_os = "windows")
}
