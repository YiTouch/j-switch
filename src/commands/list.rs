use crate::error::Result;
use crate::jdk::JdkManager;
use colored::*;

pub fn list_command() -> Result<()> {
    let mut manager = JdkManager::new()?;
    
    println!("{}", "Scanning for JDK installations...".cyan());
    manager.scan_jdks()?;
    
    let config_recently_modified = if let Ok(config_path) = crate::config::Config::config_path() {
        if let Ok(metadata) = std::fs::metadata(&config_path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = modified.elapsed() {
                    elapsed.as_secs() < 10
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };
    
    let java_home_path = std::env::var("JAVA_HOME").ok().map(|s| std::path::PathBuf::from(s));
    
    if !config_recently_modified {
        if let Some(ref home_path) = java_home_path {
            
            let system_jdk_version = manager.config().jdks.iter()
                .find(|(_, info)| &info.path == home_path)
                .map(|(key, _)| key.clone());
            
            let config_current = manager.get_current_version();
            let needs_update = match (config_current, &system_jdk_version) {
                (None, Some(_)) => true,
                (Some(config_ver), Some(system_ver)) => config_ver != system_ver,
                _ => false,
            };
            
            if needs_update {
                if let Some(version) = system_jdk_version {
                    manager.config_mut().set_current(version);
                    manager.save()?;
                }
            }
        }
    }
    
    let jdks = manager.list_jdks();
    let current_version = manager.get_current_version();
    
    if jdks.is_empty() {
        println!("{}", "No JDK installations found.".yellow());
        println!("\nPlease install JDK manually to common locations:");
        println!("  - Windows: C:\\Program Files\\Java\\");
        println!("  - macOS: /Library/Java/JavaVirtualMachines/");
        println!("  - Linux: /usr/lib/jvm/");
        println!("\nThe tool will automatically detect JDKs in these locations.");
        return Ok(());
    }
    
    println!("\n{}", "Installed JDKs:".bold()); 
    println!("{}", "=".repeat(80).bright_black()); 
    
    for (key, info) in &jdks {
        let is_current_in_config = current_version.map(|v| v == *key).unwrap_or(false);
        let is_current_in_env = java_home_path.as_ref()
            .map(|p| p == &info.path)
            .unwrap_or(false);
        let is_current = is_current_in_config || is_current_in_env;
        
        let marker = if is_current { "*".green() } else { "-".bright_black() };
        
        let status_text = if is_current_in_config && is_current_in_env {
            "(current)".green()
        } else if is_current_in_env {
            "(current - from JAVA_HOME)".yellow()
        } else if is_current_in_config {
            "(current - from config)".green()
        } else {
            "".normal()
        };
        
        println!("{} {} {}", 
            marker,
            format!("JDK {}", key).bold(),
            status_text
        );
        
        println!("  {} {}", "Version:".bright_black(),
            info.java_version.as_deref().unwrap_or("unknown"));
        
        if let Some(vendor) = &info.vendor {
            println!("  {} {}", "Vendor:".bright_black(), vendor);
        }
        
        println!("  {} {}", "Path:".bright_black(), info.path.display());
        println!();
    }
    
    println!("{}", "-".repeat(80).bright_black());
    println!("Total: {} JDK(s)", jdks.len());
    
    let has_active = jdks.iter().any(|(key, info)| {
        let is_current_in_config = current_version.map(|v| v == *key).unwrap_or(false);
        let is_current_in_env = java_home_path.as_ref()
            .map(|p| p == &info.path)
            .unwrap_or(false);
        is_current_in_config || is_current_in_env
    });
    
    if !has_active {
        println!("\n{}", "No JDK is currently active.".yellow());
        println!("Use {} to activate a JDK.", "jsh use <version>".green());
    } else if java_home_path.is_some() && current_version.is_none() {
        println!("\n{}", "Tip:".cyan().bold());
        println!("  Your JAVA_HOME is set, but not managed by jsh.");
        println!("  Run {} to let jsh manage it.", "jsh use <version>".green());
    }
    
    Ok(())
}
