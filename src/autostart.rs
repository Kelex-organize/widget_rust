use std::env;
use winreg::{enums::*, RegKey};

const APP_REG_NAME: &str = "Aniversario100-EELC";

pub fn set_enabled(enable: bool) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE | KEY_READ);

    if let Ok(run) = run {
        if enable {
            if let Ok(exe_path) = env::current_exe() {
                let _ = run.set_value(APP_REG_NAME, &exe_path.to_str().unwrap());
            }
        } else {
            let _ = run.delete_value(APP_REG_NAME);
        }
    }
}

pub fn is_enabled() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run) = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_READ) {
        run.get_value::<String, _>(APP_REG_NAME).is_ok()
    } else {
        false
    }
}
