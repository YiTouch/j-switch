use colored::Colorize;
use crate::config::Config;
use crate::downloader::adoptium::AdoptiumSource;
use crate::downloader::downloader::Downloader;
use crate::downloader::extractor::Extractor;
use crate::downloader::progress::ProgressDisplay;
use crate::downloader::traits::JdkSource;
use crate::error::{JdkError, Result};
use crate::jdk::JdkManager;

pub async fn download_command(version: &str, vendor: &str) -> Result<()> {
    println!("  Version: {}", version);
    println!("  Vendor: {}", vendor);
    println!("{}", format!("Searching for JDK {}...", version).cyan());

    let source: Box<dyn JdkSource> = match vendor.to_lowercase().as_str() {
        "temurin" | "adoptium" => Box::new(AdoptiumSource::new()),
        _ => {
            println!("{}", format!("Unknown vendor '{}', using Adoptium/Temurin", vendor).yellow());
            Box::new(AdoptiumSource::new())
        }
    };

    let version_num: u32 = version.parse()
        .map_err(|_| JdkError::InvalidVersion(version.to_string()))?;

    let package = source.find_package(version_num).await?;
    println!("\n{}", "Found package:".green().bold());
    println!("  Version:     {}", package.version);
    println!("  Vendor:      {}", package.vendor);
    println!("  Size:        {} MB", package.size / 1024 / 1024);
    println!("  Platform:    {} ({})", package.os, package.arch);
    println!("  File type:   {}", package.file_type);
    if package.is_lts {
        println!("  Support:     {}", "LTS (Long Term Support)".green());
    }

    println!("\n{}", "Downloading...".cyan());
    let downloader = Downloader::new()?;
    let filename = format!("jdk-{}-{}.{}",
                           package.major_version,
                           package.vendor,
                           package.file_type
    );

    let archive_path = downloader.download_file(
        &package.download_url,
        &filename,
        ProgressDisplay::simple_callback()
    ).await?;

    println!("{}", "[OK] Download complete".green());
    println!("\n{}", "Extracting...".cyan());
    let extractor = Extractor::new();
    let install_base = Config::config_dir()?.join("jdks");
    let jdk_path = extractor.extract(&archive_path, &install_base)?;
    println!("{}", format!("[OK] Extracted to: {}", jdk_path.display()).green());

    println!("\n{}", "Registering JDK...".cyan());
    let mut manager = JdkManager::new()?;
    manager.scan_jdks()?;

    if let Err(e) = std::fs::remove_file(&archive_path) {
        println!("{}", format!("Warning: Failed to remove archive: {}", e).yellow());
    }

    println!("\n{} {}", "[SUCCESS]".green().bold(), "JDK installed successfully!".green());
    println!("\n{}", "Next steps:".bold());
    println!("  1. List all JDKs:    {}", "j-switch list".cyan());
    println!("  2. Activate this JDK: {}", format!("j-switch use {}", version).cyan());

    Ok(())
}
