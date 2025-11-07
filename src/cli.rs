use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "A tool to manage and switch between JDK installations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all installed JDKs
    List,

    /// Show current active JDK
    Current,

    /// Switch to a specific JDK version
    Use {
        /// Version identifier (e.g., 8, 11, 17, 21)
        version: String,
    },
    
    /// Download a specific JDK version
    Download {
        /// Version to download (e.g., 17, 21)
        version: String,

        /// JDK vendor (default: temurin)
        #[arg(long, default_value = "temurin")]
        vendor: String,
    },
    
    /// Search available JDK versions for download
    Search {
        /// Optional search keyword
        keyword: Option<String>,
    },
}
