use crate::env::{EnvUpdater, get_env_updater};
use crate::error::Result;
use crate::jdk::JdkManager;
use colored::*;

pub fn use_command(version: &str) -> Result<()> {
    let mut manager = JdkManager::new()?;

    println!("{}", format!("Switching to JDK {}...", version).cyan());

    let jdk = manager.switch_jdk(version)?;

    println!("\n{}", "Updated configuration:".bold());
    println!("  {} JDK {}", "Version:".bright_black(), version.green());
    println!("  {} {}", "Path:".bright_black(), jdk.path.display());

    // Update environment variables
    println!("\n{}", "Updating environment variables...".cyan());
    let env_updater = get_env_updater();
    env_updater.update_java_home(&jdk.path)?;

    println!("\n{} {}", "[OK]".green().bold(), "Successfully switched to JDK".green());

    #[cfg(target_os = "windows")]
    {
        println!("\n{}", "Note:".yellow().bold());
        println!("  System environment variables have been updated (need administrator permission).");
        println!("  You need to restart your terminal or IDE for changes to take effect.");
        println!("  If you see permission errors, please run this tool as Administrator.");
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("\n{}", "Note:".yellow().bold());
        let rc_path = if std::env::var("SHELL").unwrap_or_default().contains("zsh") {
            "~/.zshrc"
        } else {
            "~/.bashrc"
        };
        println!("  Please run: {}", format!("source {}", rc_path).green());
    }
    
    Ok(())
}
