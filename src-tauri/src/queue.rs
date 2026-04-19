use crate::downloader::{Downloader, DownloadError};
use crate::types::{DownloadInfo, DownloadRequest, DownloadStatus, QueueStatus};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct DownloadQueue {
    downloader: Downloader,
    queue: Arc<Mutex<Vec<DownloadInfo>>>,
    active_downloads: Arc<Mutex<HashMap<String, bool>>>,
    max_concurrent: usize,
}

impl DownloadQueue {
    pub fn new(output_dir: String, max_concurrent: usize) -> Self {
        Self {
            downloader: Downloader::new(output_dir),
            queue: Arc::new(Mutex::new(Vec::new())),
            active_downloads: Arc::new(Mutex::new(HashMap::new())),
            max_concurrent,
        }
    }

    pub fn add_download(&self, request: DownloadRequest) -> Result<String, DownloadError> {
        let id = Uuid::new_v4().to_string();
        
        let (title, platform) = self.downloader.get_video_info(&request.url)?;
        
        let download_info = DownloadInfo {
            id: id.clone(),
            url: request.url.clone(),
            title,
            platform,
            format: request.format.clone(),
            quality: request.quality.clone(),
            status: DownloadStatus::Pending,
            progress: 0.0,
            file_path: None,
            file_size: None,
            created_at: chrono::Utc::now(),
            completed_at: None,
            error: None,
        };

        let mut queue = self.queue.lock().unwrap();
        queue.push(download_info);
        drop(queue);

        self.process_queue();
        
        Ok(id)
    }

    pub fn get_status(&self) -> QueueStatus {
        let queue = self.queue.lock().unwrap();
        let active = self.active_downloads.lock().unwrap();
        
        let active_count = queue.iter()
            .filter(|d| d.status == DownloadStatus::Downloading)
            .count();
        let pending_count = queue.iter()
            .filter(|d| d.status == DownloadStatus::Pending)
            .count();
        
        QueueStatus {
            active_downloads: active_count,
            pending_downloads: pending_count,
            total_downloads: queue.len(),
        }
    }

    pub fn get_all_downloads(&self) -> Vec<DownloadInfo> {
        let queue = self.queue.lock().unwrap();
        queue.clone()
    }

    pub fn cancel_download(&self, id: &str) -> Result<bool, DownloadError> {
        let mut queue = self.queue.lock().unwrap();
        
        if let Some(download) = queue.iter_mut().find(|d| d.id == id) {
            match download.status {
                DownloadStatus::Pending | DownloadStatus::Downloading => {
                    download.status = DownloadStatus::Cancelled;
                    let mut active = self.active_downloads.lock().unwrap();
                    active.remove(id);
                    drop(active);
                    self.process_queue();
                    return Ok(true);
                }
                _ => return Ok(false),
            }
        }
        
        Ok(false)
    }

    pub fn clear_completed(&self) {
        let mut queue = self.queue.lock().unwrap();
        queue.retain(|d| !matches!(d.status, DownloadStatus::Completed | DownloadStatus::Failed | DownloadStatus::Cancelled));
    }

    fn process_queue(&self) {
        let queue = self.queue.lock().unwrap();
        let active = self.active_downloads.lock().unwrap();
        
        if active.len() >= self.max_concurrent {
            return;
        }

        let pending_ids: Vec<String> = queue.iter()
            .filter(|d| d.status == DownloadStatus::Pending)
            .map(|d| d.id.clone())
            .collect();
        
        drop(queue);
        drop(active);

        for id in pending_ids {
            let active = self.active_downloads.lock().unwrap();
            if active.len() >= self.max_concurrent {
                drop(active);
                break;
            }
            drop(active);

            self.start_download(&id);
        }
    }

    fn start_download(&self, id: &str) {
        let queue_arc = self.queue.clone();
        let active_arc = self.active_downloads.clone();
        let id = id.to_string();
        
        {
            let mut active = self.active_downloads.lock().unwrap();
            active.insert(id.clone(), true);
        }

        {
            let mut queue = self.queue.lock().unwrap();
            if let Some(download) = queue.iter_mut().find(|d| d.id == id) {
                download.status = DownloadStatus::Downloading;
            }
        }

        tokio::spawn(async move {
            let request = {
                let queue = queue_arc.lock().unwrap();
                queue.iter().find(|d| d.id == id).map(|d| DownloadRequest {
                    url: d.url.clone(),
                    format: d.format.clone(),
                    quality: d.quality.clone(),
                })
            };

            if let Some(req) = request {
                let downloader = Downloader::new("./downloads".to_string());
                match downloader.download(req, |progress| {
                    let mut queue = queue_arc.lock().unwrap();
                    if let Some(download) = queue.iter_mut().find(|d| d.id == id) {
                        download.progress = progress;
                    }
                }) {
                    Ok(result) => {
                        let mut queue = queue_arc.lock().unwrap();
                        if let Some(download) = queue.iter_mut().find(|d| d.id == id) {
                            *download = result;
                        }
                    }
                    Err(_) => {
                        let mut queue = queue_arc.lock().unwrap();
                        if let Some(download) = queue.iter_mut().find(|d| d.id == id) {
                            download.status = DownloadStatus::Failed;
                            download.error = Some("Download failed".to_string());
                        }
                    }
                }
            }

            let mut active = active_arc.lock().unwrap();
            active.remove(&id);
        });
    }
}
