use crate::error::{JdkError, Result};
use crate::jdk::JdkManager;
use colored::*;

pub fn current_command() -> Result<()> {
    let manager = JdkManager::new()?;

    let current = manager.get_current()
        .ok_or(JdkError::NoActiveJdk)?;

    let version_key = manager.get_current_version().unwrap();

    println!("{}", "Current JDK:".bold());
    println!("{}", "=".repeat(60).bright_black());
    println!("{} {}", "Version:".bright_black(), format!("JDK {}", version_key).green().bold());
    println!("{} {}", "Full Version:".bright_black(),
        current.java_version.as_deref().unwrap_or("unknown"));

    if let Some(vendor) = &current.vendor {
        println!("{} {}", "Vendor:".bright_black(), vendor);
    }
    
    println!("{} {}", "Path:".bright_black(), current.path.display());
    println!("{}", "=".repeat(60).bright_black());

    // Verify environment
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        if java_home == current.path.to_string_lossy() {
            println!("\n{} JAVA_HOME is correctly set", "[OK]".green());
        } else {
            println!("\n{} JAVA_HOME mismatch:", "[!]".yellow());
            println!("  Expected: {}", current.path.display());
            println!("  Current:  {}", java_home);
            println!("  {}", "Try restarting your terminal or running the use command again.".yellow());
        }
    } else {
        println!("\n{} JAVA_HOME is not set", "[!]".yellow());
    }
    
    Ok(())
}
