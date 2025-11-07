use crate::env::EnvUpdater;
use crate::error::{JdkError, Result};
use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct UnixEnvUpdater;

impl UnixEnvUpdater {
    pub fn new() -> Self {
        Self
    }

    fn get_shell_rc_path() -> Result<std::path::PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| JdkError::EnvError("Cannot find home directory".to_string()))?;

        // Try to detect shell
        if let Ok(shell) = std::env::var("SHELL") {
            if shell.contains("zsh") {
                return Ok(home.join(".zshrc"));
            } else if shell.contains("bash") {
                return Ok(home.join(".bashrc"));
            }
        }

        // Default to .bashrc
        Ok(home.join(".bashrc"))
    }

    fn update_shell_rc(&self, java_home: &Path) -> Result<()> {
        let rc_path = Self::get_shell_rc_path()?;
        let java_home_str = java_home.to_string_lossy();

        // Read existing content
        let mut lines = Vec::new();
        let mut found_java_home = false;
        let mut found_path = false;

        if rc_path.exists() {
            let file = std::fs::File::open(&rc_path)
                .map_err(|e| JdkError::IoError(e))?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line.map_err(|e| JdkError::IoError(e))?;
                if line.contains("export JAVA_HOME=") && line.contains("# jsh managed") {
                    lines.push(format!("export JAVA_HOME=\"{}\"  # jsh managed", java_home_str));
                    found_java_home = true;
                } else if line.contains("export PATH=") && line.contains("$JAVA_HOME/bin") && line.contains("# jsh managed") {
                    lines.push(format!("export PATH=\"$JAVA_HOME/bin:$PATH\"  # jsh managed"));
                    found_path = true;
                } else {
                    lines.push(line);
                }
            }
        }

        // Add new entries if not found
        if !found_java_home {
            lines.push(format!("\n# jsh managed - do not edit manually"));
            lines.push(format!("export JAVA_HOME=\"{}\"  # jsh managed", java_home_str));
        }
        if !found_path {
            lines.push(format!("export PATH=\"$JAVA_HOME/bin:$PATH\"  # jsh managed"));
        }

        // Write back
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&rc_path)
            .map_err(|e| JdkError::IoError(e))?;

        for line in lines {
            writeln!(file, "{}", line).map_err(|e| JdkError::IoError(e))?;
        }

        println!("[OK] Updated {}", rc_path.display());
        println!("  Please run: source {}", rc_path.display());

        Ok(())
    }
}

impl EnvUpdater for UnixEnvUpdater {
    fn update_java_home(&self, path: &Path) -> Result<()> {
        self.update_shell_rc(path)?;
        Ok(())
    }

    fn update_path(&self, _java_home: &Path) -> Result<()> {
        // Handled together in update_shell_rc
        Ok()
    }
}
