use crate::types::{DownloadInfo, DownloadRequest, DownloadStatus, Platform, VideoFormat, VideoQuality};
use chrono::Utc;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("yt-dlp not found. Please install yt-dlp: https://github.com/yt-dlp/yt-dlp")]
    YtDlpNotFound,
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    #[error("Process error: {0}")]
    ProcessError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct Downloader {
    output_dir: String,
}

impl Downloader {
    pub fn new(output_dir: String) -> Self {
        Self { output_dir }
    }

    fn get_yt_dlp_executable(&self) -> Result<std::path::PathBuf, DownloadError> {
        // First check for yt-dlp in application directory
        if let Ok(exe_path) = std::env::current_exe() {
            let app_dir = exe_path.parent();
            if let Some(dir) = app_dir {
                let local_yt_dlp = dir.join("yt-dlp.exe");
                if local_yt_dlp.exists() {
                    return Ok(local_yt_dlp);
                }
            }
        }

        // Fall back to system PATH
        if which::which("yt-dlp").is_ok() {
            Ok(std::path::PathBuf::from("yt-dlp"))
        } else {
            Err(DownloadError::YtDlpNotFound)
        }
    }

    pub fn check_yt_dlp_installed(&self) -> Result<bool, DownloadError> {
        match self.get_yt_dlp_executable() {
            Ok(path) => {
                let result = Command::new(&path)
                    .arg("--version")
                    .output();

                match result {
                    Ok(output) => Ok(output.status.success()),
                    Err(_) => Ok(false),
                }
            }
            Err(_) => Ok(false),
        }
    }

    pub fn get_video_info(&self, url: &str) -> Result<(String, Platform), DownloadError> {
        if !self.is_valid_url(url) {
            return Err(DownloadError::InvalidUrl(url.to_string()));
        }

        let yt_dlp = self.get_yt_dlp_executable()?;
        let output = Command::new(&yt_dlp)
            .arg("--get-title")
            .arg(url)
            .output()
            .map_err(|_| DownloadError::YtDlpNotFound)?;

        if !output.status.success() {
            return Err(DownloadError::DownloadFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let title = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        let platform = Platform::from_url(url);

        Ok((title, platform))
    }

    pub fn get_available_formats(&self, url: &str) -> Result<Vec<String>, DownloadError> {
        let yt_dlp = self.get_yt_dlp_executable()?;
        let output = Command::new(&yt_dlp)
            .arg("--list-formats")
            .arg(url)
            .output()
            .map_err(|_| DownloadError::YtDlpNotFound)?;

        if !output.status.success() {
            return Err(DownloadError::DownloadFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let formats_output = String::from_utf8_lossy(&output.stdout);
        let formats: Vec<String> = formats_output
            .lines()
            .filter(|line| line.contains("mp4") || line.contains("audio only"))
            .map(|line| line.to_string())
            .collect();

        Ok(formats)
    }

    pub fn download(
        &self,
        request: DownloadRequest,
        progress_callback: impl Fn(f32),
    ) -> Result<DownloadInfo, DownloadError> {
        let id = Uuid::new_v4().to_string();
        let (title, platform) = self.get_video_info(&request.url)?;
        let created_at = Utc::now();

        let mut download_info = DownloadInfo {
            id: id.clone(),
            url: request.url.clone(),
            title: title.clone(),
            platform,
            format: request.format.clone(),
            quality: request.quality.clone(),
            status: DownloadStatus::Downloading,
            progress: 0.0,
            file_path: None,
            file_size: None,
            created_at,
            completed_at: None,
            error: None,
        };

        let format_arg = match request.format {
            VideoFormat::Mp4 => "-f",
            VideoFormat::Mp3 => "-x",
        };

        let quality_arg = match request.quality {
            VideoQuality::Best => "best",
            VideoQuality::High => "best[height<=1080]",
            VideoQuality::Medium => "best[height<=720]",
            VideoQuality::Low => "worst",
        };

        let output_template = format!(
            "{}/%(title)s.%(ext)s",
            self.output_dir
        );

        let yt_dlp = self.get_yt_dlp_executable()?;
        let mut cmd = Command::new(&yt_dlp);
        cmd.arg(format_arg)
            .arg(quality_arg)
            .arg("-o")
            .arg(&output_template)
            .arg("--newline");

        // Add GPU acceleration if enabled and format is MP4 (video)
        if request.gpu_acceleration && request.format == VideoFormat::Mp4 {
            if let Some(gpu_args) = self.get_gpu_acceleration_args() {
                cmd.arg("--postprocessor-args").arg(&gpu_args);
            }
        }

        cmd.arg(&request.url);

        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DownloadError::ProcessError(e.to_string()))?;

        let stderr = child.stderr.take().ok_or_else(|| {
            DownloadError::ProcessError("Failed to capture stderr".to_string())
        })?;

        let reader = BufReader::new(stderr);
        let mut last_progress = 0.0;

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if let Some(progress) = self.parse_progress(&line) {
                        if progress > last_progress {
                            progress_callback(progress);
                            download_info.progress = progress;
                            last_progress = progress;
                        }
                    }
                }
                Err(_) => break,
            }
        }

        let status = child.wait().map_err(|e| DownloadError::ProcessError(e.to_string()))?;

        if status.success() {
            download_info.status = DownloadStatus::Completed;
            download_info.progress = 100.0;
            download_info.completed_at = Some(Utc::now());
            download_info.file_path = Some(format!("{}/{}.{}", 
                self.output_dir, 
                title.replace(|c: char| !c.is_alphanumeric() && c != ' ' && c != '-', "_"),
                match request.format {
                    VideoFormat::Mp4 => "mp4",
                    VideoFormat::Mp3 => "mp3",
                }
            ));
        } else {
            download_info.status = DownloadStatus::Failed;
            download_info.error = Some("Download failed".to_string());
            return Err(DownloadError::DownloadFailed("Download process failed".to_string()));
        }

        Ok(download_info)
    }

    fn parse_progress(&self, line: &str) -> Option<f32> {
        if line.contains("[download]") && line.contains("%") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for part in parts {
                if part.ends_with('%') {
                    let percent_str = part.trim_end_matches('%');
                    return percent_str.parse::<f32>().ok();
                }
            }
        }
        None
    }

    fn is_valid_url(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    fn get_gpu_acceleration_args(&self) -> Option<String> {
        // Try to detect available GPU and return appropriate FFmpeg arguments
        // NVIDIA (NVENC)
        if self.check_nvidia_gpu() {
            return Some("ffmpeg:-c:v h264_nvenc -preset fast".to_string());
        }

        // AMD (AMF)
        if self.check_amd_gpu() {
            return Some("ffmpeg:-c:v h264_amf -preset speed".to_string());
        }

        // Intel (Quick Sync)
        if self.check_intel_gpu() {
            return Some("ffmpeg:-c:v h264_qsv -preset faster".to_string());
        }

        // macOS (VideoToolbox)
        if cfg!(target_os = "macos") {
            return Some("ffmpeg:-c:v h264_videotoolbox".to_string());
        }

        None
    }

    fn check_nvidia_gpu(&self) -> bool {
        // Check for NVIDIA GPU by trying to run nvidia-smi
        #[cfg(target_os = "windows")]
        {
            let result = Command::new("nvidia-smi")
                .arg("--query-gpu=name")
                .arg("--format=csv,noheader")
                .output();
            result.is_ok() && result.unwrap().status.success()
        }

        #[cfg(not(target_os = "windows"))]
        {
            let result = Command::new("nvidia-smi")
                .arg("--query-gpu=name")
                .arg("--format=csv,noheader")
                .output();
            result.is_ok() && result.unwrap().status.success()
        }
    }

    fn check_amd_gpu(&self) -> bool {
        // Check for AMD GPU by looking for AMD/ATI in lspci (Linux) or similar
        #[cfg(target_os = "windows")]
        {
            // On Windows, check if AMD GPU is present via wmic or similar
            let result = Command::new("wmic")
                .args(&["path", "win32_VideoController", "get", "name"])
                .output();
            if result.is_ok() {
                let output = String::from_utf8_lossy(&result.unwrap().stdout);
                output.to_lowercase().contains("amd") || output.to_lowercase().contains("radeon")
            } else {
                false
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let result = Command::new("lspci")
                .arg("-nnk")
                .output();
            if result.is_ok() {
                let output = String::from_utf8_lossy(&result.unwrap().stdout);
                output.to_lowercase().contains("amd") || output.to_lowercase().contains("radeon") ||
                output.to_lowercase().contains("ati")
            } else {
                false
            }
        }
    }

    fn check_intel_gpu(&self) -> bool {
        // Check for Intel GPU
        #[cfg(target_os = "windows")]
        {
            let result = Command::new("wmic")
                .args(&["path", "win32_VideoController", "get", "name"])
                .output();
            if result.is_ok() {
                let output = String::from_utf8_lossy(&result.unwrap().stdout);
                output.to_lowercase().contains("intel") && 
                (output.to_lowercase().contains("graphics") || output.to_lowercase().contains("iris") || output.to_lowercase().contains("arc"))
            } else {
                false
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let result = Command::new("lspci")
                .arg("-nnk")
                .output();
            if result.is_ok() {
                let output = String::from_utf8_lossy(&result.unwrap().stdout);
                output.to_lowercase().contains("intel") && 
                (output.to_lowercase().contains("graphics") || output.to_lowercase().contains("iris") || output.to_lowercase().contains("arc"))
            } else {
                false
            }
        }
    }
}
