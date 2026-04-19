use crate::types::DownloadInfo;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct DownloadHistory {
    history_file: PathBuf,
    history: Arc<Mutex<Vec<DownloadInfo>>>,
}

impl DownloadHistory {
    pub fn new(history_file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let history = Self::load_from_file(&history_file)?;
        
        Ok(Self {
            history_file,
            history: Arc::new(Mutex::new(history)),
        })
    }

    fn load_from_file(path: &PathBuf) -> Result<Vec<DownloadInfo>, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let history: Vec<DownloadInfo> = serde_json::from_reader(reader)?;
        Ok(history)
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.history_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = File::create(&self.history_file)?;
        let writer = BufWriter::new(file);
        let history = self.history.lock().unwrap();
        serde_json::to_writer(writer, &*history)?;
        Ok(())
    }

    pub fn add(&self, download: DownloadInfo) -> Result<(), Box<dyn std::error::Error>> {
        let mut history = self.history.lock().unwrap();
        history.push(download);
        drop(history);
        self.save_to_file()
    }

    pub fn get_all(&self) -> Vec<DownloadInfo> {
        let history = self.history.lock().unwrap();
        history.clone()
    }

    pub fn remove(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut history = self.history.lock().unwrap();
        let initial_len = history.len();
        history.retain(|d| d.id != id);
        let removed = history.len() < initial_len;
        drop(history);
        
        if removed {
            self.save_to_file()?;
        }
        
        Ok(removed)
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut history = self.history.lock().unwrap();
        history.clear();
        drop(history);
        self.save_to_file()
    }

    pub fn get_by_platform(&self, platform: &str) -> Vec<DownloadInfo> {
        let history = self.history.lock().unwrap();
        history.iter()
            .filter(|d| format!("{:?}", d.platform).to_lowercase() == platform.to_lowercase())
            .cloned()
            .collect()
    }
}
