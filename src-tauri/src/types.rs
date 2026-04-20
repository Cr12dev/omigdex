use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub format: VideoFormat,
    pub quality: VideoQuality,
    pub gpu_acceleration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VideoFormat {
    Mp4,
    Mp3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoQuality {
    Best,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadInfo {
    pub id: String,
    pub url: String,
    pub title: String,
    pub platform: Platform,
    pub format: VideoFormat,
    pub quality: VideoQuality,
    pub status: DownloadStatus,
    pub progress: f32,
    pub file_path: Option<String>,
    pub file_size: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Youtube,
    Instagram,
    Tiktok,
    Pinterest,
    Unknown,
}

impl Platform {
    pub fn from_url(url: &str) -> Self {
        if url.contains("youtube.com") || url.contains("youtu.be") {
            Platform::Youtube
        } else if url.contains("instagram.com") {
            Platform::Instagram
        } else if url.contains("tiktok.com") {
            Platform::Tiktok
        } else if url.contains("pinterest.com") {
            Platform::Pinterest
        } else {
            Platform::Unknown
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatOption {
    pub format_id: String,
    pub ext: String,
    pub quality: String,
    pub filesize: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueStatus {
    pub active_downloads: usize,
    pub pending_downloads: usize,
    pub total_downloads: usize,
}
