use crate::config::Config;
use crate::error::{JdkError, Result};
use futures_util::StreamExt;
use reqwest::{Client};
use std::path::{PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct Downloader {
    client: Client,
    download_dir: PathBuf,
}

impl Downloader {
    pub fn new() -> Result<Self> {
        let download_dir = Config::config_dir()?.join("downloads");
        std::fs::create_dir_all(&download_dir).map_err(|e| JdkError::IoError(e))?;
        Ok(Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(600))
                .build()
                .map_err(|e| JdkError::DownloadError(e.to_string()))?,
            download_dir: download_dir.clone(),
        })
    }

    pub async fn download_file<F>(
        &self,
        url: &str,
        filename: &str,
        on_progress: F,
    ) -> Result<PathBuf>
    where
        F: Fn(u64, u64) + Send + 'static,
    {
        let target_path = self.download_dir.join(filename);
        // file exists
        if target_path.exists() {
            println!("File already exists: {}", target_path.display());
            println!("Using existing file...");
            return Ok(target_path);
        }

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| JdkError::NetworkError(e.to_string()))?;

        let total_size = response
            .content_length()
            .ok_or_else(|| JdkError::DownloadError("Unknown file size".to_string()))?;
        let mut file = File::create(&target_path)
            .await
            .map_err(|e| JdkError::IoError(e))?;

        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| JdkError::NetworkError(e.to_string()))?;
            file.write_all(&chunk)
                .await
                .map_err(|e| JdkError::IoError(e))?;
            downloaded += chunk.len() as u64;
            on_progress(downloaded, total_size);
        }
        file.flush().await.map_err(|e| JdkError::IoError(e))?;
        Ok(target_path)
    }
}
