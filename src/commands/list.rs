use crate::error::Result;
use crate::jdk::JdkManager;
use colored::*;

pub fn list_command() -> Result<()> {
    let mut manager = JdkManager::new()?;
    
    println!("{}", "Scanning for JDK installations...".cyan());
    manager.scan_jdks()?;
    
    let config_recently_modified = crate::config::Config::config_path()
        .ok()
        .and_then(|p| std::fs::metadata(&p).ok())
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.elapsed().ok())
        .map(|e| e.as_secs() < 10)
        .unwrap_or(false);
    
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
        
        // Priority: environment variable takes precedence
        let is_current = is_current_in_env || (is_current_in_config && java_home_path.is_none());
        
        let marker = if is_current { "*".green() } else { "-".bright_black() };
        
        let status_text = if is_current_in_env && is_current_in_config {
            "(active)".green()
        } else if is_current_in_env {
            "(active - environment only)".yellow()
        } else if is_current_in_config && java_home_path.is_some() {
            // Config says this is current, but env says otherwise
            "(config mismatch)".red()
        } else if is_current_in_config {
            "(config - env not set)".yellow()
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
    
    let has_active_in_env = jdks.iter().any(|(_, info)| {
        java_home_path.as_ref().map(|p| p == &info.path).unwrap_or(false)
    });
    
    let has_config_mismatch = if let (Some(env_path), Some(config_ver)) = (&java_home_path, current_version) {
        jdks.iter()
            .find(|(k, _)| k == &config_ver)
            .map(|(_, info)| &info.path != env_path)
            .unwrap_or(false)
    } else {
        false
    };
    
    if !has_active_in_env && java_home_path.is_none() && current_version.is_none() {
        println!("\n{}", "No JDK is currently active.".yellow());
        println!("Use {} to activate a JDK.", "jsh use <version>".green());
    } else if has_config_mismatch {
        println!("\n{}", "Warning:".yellow().bold());
        println!("  JAVA_HOME environment variable does not match jsh config.");
        println!("  Current JDK is determined by JAVA_HOME (shown with * above).");
        println!("  Run {} to sync config with environment.", "jsh use <version>".green());
    } else if java_home_path.is_some() && current_version.is_none() {
        println!("\n{}", "Tip:".cyan().bold());
        println!("  Your JAVA_HOME is set, but not managed by jsh.");
        println!("  Run {} to let jsh manage it.", "jsh use <version>".green());
    } else if java_home_path.is_none() && current_version.is_some() {
        println!("\n{}", "Warning:".yellow().bold());
        println!("  JAVA_HOME is not set in your environment.");
        println!("  Run {} again to set environment variables.", "jsh use <version>".green());
    }
    
    Ok(())
}
