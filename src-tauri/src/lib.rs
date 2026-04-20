mod downloader;
mod queue;
mod history;
mod types;

use downloader::{Downloader, DownloadError};
use queue::DownloadQueue;
use history::DownloadHistory;
use types::{DownloadRequest, DownloadInfo, QueueStatus, VideoFormat, VideoQuality};
use std::sync::{Arc, Mutex};

#[tauri::command]
fn check_yt_dlp() -> Result<bool, String> {
    let downloader = Downloader::new("./downloads".to_string());
    downloader.check_yt_dlp_installed()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_video_info(url: String) -> Result<(String, String), String> {
    let downloader = Downloader::new("./downloads".to_string());
    let (title, platform) = downloader.get_video_info(&url)
        .map_err(|e| e.to_string())?;
    Ok((title, format!("{:?}", platform).to_lowercase()))
}

#[tauri::command]
fn get_available_formats(url: String) -> Result<Vec<String>, String> {
    let downloader = Downloader::new("./downloads".to_string());
    downloader.get_available_formats(&url)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn download_video(
    url: String,
    format: String,
    quality: String,
    state: tauri::State<'_, Arc<Mutex<DownloadQueue>>>,
) -> Result<String, String> {
    let video_format = match format.to_lowercase().as_str() {
        "mp4" => VideoFormat::Mp4,
        "mp3" => VideoFormat::Mp3,
        _ => return Err("Invalid format".to_string()),
    };

    let video_quality = match quality.to_lowercase().as_str() {
        "best" => VideoQuality::Best,
        "high" => VideoQuality::High,
        "medium" => VideoQuality::Medium,
        "low" => VideoQuality::Low,
        _ => VideoQuality::Best,
    };

    let request = DownloadRequest {
        url,
        format: video_format,
        quality: video_quality,
    };

    let queue = state.lock().unwrap();
    let id = queue.add_download(request)
        .map_err(|e| e.to_string())?;
    
    Ok(id)
}

#[tauri::command]
fn get_queue_status(state: tauri::State<'_, Arc<Mutex<DownloadQueue>>>) -> QueueStatus {
    let queue = state.lock().unwrap();
    queue.get_status()
}

#[tauri::command]
fn get_all_downloads(state: tauri::State<'_, Arc<Mutex<DownloadQueue>>>) -> Vec<DownloadInfo> {
    let queue = state.lock().unwrap();
    queue.get_all_downloads()
}

#[tauri::command]
fn cancel_download(id: String, state: tauri::State<'_, Arc<Mutex<DownloadQueue>>>) -> Result<bool, String> {
    let queue = state.lock().unwrap();
    queue.cancel_download(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_completed(state: tauri::State<'_, Arc<Mutex<DownloadQueue>>>) {
    let queue = state.lock().unwrap();
    queue.clear_completed();
}

#[tauri::command]
fn get_history(state: tauri::State<'_, Arc<Mutex<DownloadHistory>>>) -> Vec<DownloadInfo> {
    let history = state.lock().unwrap();
    history.get_all()
}

#[tauri::command]
fn add_to_history(download: DownloadInfo, state: tauri::State<'_, Arc<Mutex<DownloadHistory>>>) -> Result<(), String> {
    let history = state.lock().unwrap();
    history.add(download)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_from_history(id: String, state: tauri::State<'_, Arc<Mutex<DownloadHistory>>>) -> Result<bool, String> {
    let history = state.lock().unwrap();
    history.remove(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_history(state: tauri::State<'_, Arc<Mutex<DownloadHistory>>>) -> Result<(), String> {
    let history = state.lock().unwrap();
    history.clear()
        .map_err(|e| e.to_string())
}

// Actualmente se trabaja en esto
#[tauri::command]
fn open_file(file_path: String) -> Result<(), String> {
    opener::open(file_path)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let queue = Arc::new(Mutex::new(DownloadQueue::new("./downloads".to_string(), 3)));
    let history = Arc::new(Mutex::new(
        DownloadHistory::new("./history.json".into()).expect("Failed to initialize history")
    ));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(queue)
        .manage(history)
        .invoke_handler(tauri::generate_handler![
            check_yt_dlp,
            get_video_info,
            get_available_formats,
            download_video,
            get_queue_status,
            get_all_downloads,
            cancel_download,
            clear_completed,
            get_history,
            add_to_history,
            remove_from_history,
            clear_history,
            open_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
