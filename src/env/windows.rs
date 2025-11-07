use crate::env::EnvUpdater;
use crate::error::{JdkError, Result};
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

pub struct WindowsEnvUpdater;

impl WindowsEnvUpdater {
    pub fn new() -> Self {
        Self
    }

    fn get_environment_key() -> Result<RegKey> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", KEY_READ | KEY_WRITE)
            .map_err(|e| JdkError::EnvError(format!("Failed to open registry key (need administrator permission): {}", e)))
    }

    fn broadcast_environment_change() -> Result<()> {
        use windows::Win32::Foundation::*;
        use windows::Win32::UI::WindowsAndMessaging::*;

        unsafe {
            let result = SendMessageTimeoutW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                WPARAM(0),
                LPARAM(
                    windows::core::PCWSTR::from_raw(
                        "Environment\0".encode_utf16().collect::<Vec<_>>().as_ptr(),
                    )
                    .as_ptr() as isize,
                ),
                SMTO_ABORTIFHUNG,
                5000,
                None,
            );

            if result.0 == 0 {
                eprintln!("Warning: Failed to broadcast environment change. You may need to restart your terminal.");
            }
        }

        Ok(())
    }
}

impl EnvUpdater for WindowsEnvUpdater {
    fn update_java_home(&self, path: &Path) -> Result<()> {
        let env_key = Self::get_environment_key()?;
        
        let path_str = path.to_string_lossy().to_string();
        env_key
            .set_value("JAVA_HOME", &path_str)
            .map_err(|e| JdkError::EnvError(format!("Failed to set JAVA_HOME: {}", e)))?;

        println!("[OK] Set JAVA_HOME to: {}", path_str);
        
        // Update PATH using the same registry key
        self.update_path_with_key(&env_key, path)?;
        Self::broadcast_environment_change()?;

        Ok(())
    }
    
    fn update_path_with_key(&self, env_key: &RegKey, java_home: &Path) -> Result<()> {
        let path_value: String = env_key
            .get_value("Path")
            .unwrap_or_else(|_| String::new());

        let java_bin = java_home.join("bin").to_string_lossy().to_string();
        
        // Remove old Java paths
        let mut paths: Vec<String> = path_value
            .split(';')
            .filter(|p| !p.is_empty())
            .map(|s| s.to_string())
            .collect();

        // Remove existing Java bin paths (more precise matching)
        let java_home_str = java_home.to_string_lossy().to_lowercase();
        paths.retain(|p| {
            let p_lower = p.to_lowercase();
            // Only remove paths that are actually within a JDK installation
            !(p_lower.contains(&java_home_str) || 
              (p_lower.contains("\\bin") && (p_lower.contains("\\jdk") || p_lower.contains("\\jre"))))
        });

        // Add new Java bin path at the beginning
        paths.insert(0, java_bin.clone());

        let new_path = paths.join(";");
        
        env_key
            .set_value("Path", &new_path)
            .map_err(|e| JdkError::EnvError(format!("Failed to update PATH: {}", e)))?;

        println!("[OK] Updated PATH to include: {}", java_bin);

        Ok(())
    }
}
