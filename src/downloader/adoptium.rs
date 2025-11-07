use async_trait::async_trait;
use crate::downloader::traits::{JdkPackage, JdkSource, detect_arch, detect_os, get_file_type};
use crate::error::JdkError;
use crate::error::Result;
use reqwest::Client;
use serde::Deserialize;

pub struct AdoptiumSource {
    client: Client,
}

impl AdoptiumSource {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
        }
    }

    /// version + os + arch + lts to find jdk info
    async fn fetch_version_package(
        &self,
        version: u32,
        os: &str,
        arch: &str,
        lts_versions: &[u32],
    ) -> Result<JdkPackage> {
        let url = format!(
            "https://api.adoptium.net/v3/assets/latest/{}/hotspot?os={}&architecture={}&image_type=jdk",
            version, os, arch
        );
        let mut response: Vec<AssetResponse> = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| JdkError::NetworkError(e.to_string()))?
            .json()
            .await
            .map_err(|e| JdkError::NetworkError(e.to_string()))?;

        let asset = response
            .pop()
            .ok_or_else(|| JdkError::PackageNotFound(version.to_string()))?;

        Ok(JdkPackage {
            version: asset.version.semver.clone(),
            major_version: asset.version.major,
            vendor: "temurin".to_string(),
            os: asset.binary.os.clone(),
            arch: asset.binary.architecture.clone(),
            download_url: asset.binary.package.link.clone(),
            size: asset.binary.package.size,
            file_type: get_file_type(&os).to_string(),
            is_lts: lts_versions.contains(&version),
            checksum: Some(asset.binary.package.checksum),
        })
    }
}

// ============adoptium api structs============
#[derive(Deserialize, Debug)]
pub struct AvailableReleases {
    available_releases: Vec<u32>,
    available_lts_releases: Vec<u32>,
}
#[derive(Deserialize, Debug)]
struct AssetResponse {
    binary: Binary,
    version: Version,
    // release_name: String,
}
#[derive(Deserialize, Debug)]
struct Version {
    major: u32,
    // minor: u32,
    // security: u32,
    // build: u32,
    semver: String,
}
#[derive(Deserialize, Debug)]
struct Binary {
    architecture: String,
    os: String,
    package: Package,
}
#[derive(Deserialize, Debug)]
struct Package {
    link: String,
    size: u64,
    // name: String,
    checksum: String,
}
// ============adoptium api structs============

#[async_trait]
impl JdkSource for AdoptiumSource {
    fn name(&self) -> &str {
        "Eclipse Adoptium (Temurin)"
    }

    async fn fetch_version(&self) -> Result<Vec<JdkPackage>> {
        let release_url = "https://api.adoptium.net/v3/info/available_releases";
        let releases: AvailableReleases = self
            .client
            .get(release_url)
            .send()
            .await
            .map_err(|e| JdkError::NetworkError(e.to_string()))?
            .json()
            .await
            .map_err(|e| JdkError::NetworkError(e.to_string()))?;

        let os = detect_os();
        let arch = detect_arch();

        let mut packages = Vec::new();
        for version in releases.available_releases {
            if let Ok(pkg) = self
                .fetch_version_package(version, &os, &arch, &releases.available_lts_releases)
                .await
            {
                packages.push(pkg);
            }
        }
        Ok(packages)
    }
}
