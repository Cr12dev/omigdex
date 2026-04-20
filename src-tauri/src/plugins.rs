use mlua::{Lua, Function, Table, Value};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::types::{DownloadInfo, DownloadRequest, VideoFormat, VideoQuality};
use crate::downloader::Downloader;
use crate::queue::DownloadQueue;
use crate::history::DownloadHistory;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Plugin load error: {0}")]
    LoadError(String),
    #[error("Plugin execution error: {0}")]
    ExecutionError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Lua error: {0}")]
    LuaError(#[from] mlua::Error),
}

pub struct Plugin {
    name: String,
    lua: Lua,
    enabled: bool,
}

impl Plugin {
    pub fn new(name: String, lua: Lua) -> Self {
        Self {
            name,
            lua,
            enabled: true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

pub struct PluginManager {
    plugins: Arc<Mutex<Vec<Plugin>>>,
    downloader: Arc<Mutex<Downloader>>,
    queue: Arc<Mutex<DownloadQueue>>,
    history: Arc<Mutex<DownloadHistory>>,
    plugins_dir: PathBuf,
}

impl PluginManager {
    pub fn new(
        downloader: Arc<Mutex<Downloader>>,
        queue: Arc<Mutex<DownloadQueue>>,
        history: Arc<Mutex<DownloadHistory>>,
        plugins_dir: PathBuf,
    ) -> Self {
        // Create plugins directory if it doesn't exist
        if !plugins_dir.exists() {
            fs::create_dir_all(&plugins_dir).expect("Failed to create plugins directory");
        }

        Self {
            plugins: Arc::new(Mutex::new(Vec::new())),
            downloader,
            queue,
            history,
            plugins_dir,
        }
    }

    pub fn load_plugins(&self) -> Result<(), PluginError> {
        let mut plugins = self.plugins.lock().unwrap();
        plugins.clear();

        if !self.plugins_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.plugins_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("lua") {
                let plugin_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                match self.load_plugin(&path) {
                    Ok(plugin) => {
                        println!("Loaded plugin: {}", plugin_name);
                        plugins.push(plugin);
                    }
                    Err(e) => {
                        eprintln!("Failed to load plugin {}: {}", plugin_name, e);
                    }
                }
            }
        }

        Ok(())
    }

    fn load_plugin(&self, path: &PathBuf) -> Result<Plugin, PluginError> {
        let lua = Lua::new();

        // Expose API functions to Lua
        self.expose_api(&lua)?;

        // Load the plugin file
        let plugin_code = fs::read_to_string(path)?;
        lua.load(&plugin_code).exec()?;

        // Call plugin init if exists
        if let Ok(init) = lua.globals().get::<Function>("init") {
            init.call::<_, ()>(())?;
        }

        Ok(Plugin::new(
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            lua,
        ))
    }

    fn expose_api(&self, lua: &Lua) -> Result<(), PluginError> {
        let globals = lua.globals();

        // Create omigdex API table
        let omigdex = lua.create_table()?;

        // Logger
        omigdex.set("log", lua.create_function(|_, msg: String| {
            println!("[Plugin] {}", msg);
            Ok(())
        })?)?;

        // Download functions
        let downloader = Arc::clone(&self.downloader);
        omigdex.set("get_video_info", lua.create_function(move |_, url: String| {
            let dl = downloader.lock().unwrap();
            match dl.get_video_info(&url) {
                Ok((title, platform)) => {
                    let result = lua.create_table()?;
                    result.set("title", title)?;
                    result.set("platform", platform.to_lowercase())?;
                    Ok(result)
                }
                Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
            }
        })?)?;

        // Queue functions
        let queue = Arc::clone(&self.queue);
        omigdex.set("get_queue_status", lua.create_function(move |_, ()| {
            let q = queue.lock().unwrap();
            let status = q.get_status();
            let result = lua.create_table()?;
            result.set("active_downloads", status.active_downloads)?;
            result.set("pending_downloads", status.pending_downloads)?;
            result.set("total_downloads", status.total_downloads)?;
            Ok(result)
        })?)?;

        // History functions
        let history = Arc::clone(&self.history);
        omigdex.set("get_history", lua.create_function(move |_, ()| {
            let h = history.lock().unwrap();
            let downloads = h.get_all();
            let result = lua.create_table()?;
            for (i, download) in downloads.iter().enumerate() {
                let dl_table = lua.create_table()?;
                dl_table.set("id", &download.id)?;
                dl_table.set("url", &download.url)?;
                dl_table.set("title", &download.title)?;
                dl_table.set("platform", format!("{:?}", download.platform).to_lowercase())?;
                dl_table.set("format", format!("{:?}", download.format).to_lowercase())?;
                dl_table.set("status", format!("{:?}", download.status).to_lowercase())?;
                dl_table.set("progress", download.progress)?;
                dl_table.set("file_path", download.file_path.clone().unwrap_or_default())?;
                result.set(i + 1, dl_table)?;
            }
            Ok(result)
        })?)?;

        // Utility functions
        omigdex.set("read_file", lua.create_function(|_, path: String| {
            fs::read_to_string(&path).map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        })?)?;

        omigdex.set("write_file", lua.create_function(|_, (path, content): (String, String)| {
            fs::write(&path, content).map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        })?)?;

        globals.set("omigdex", omigdex)?;

        Ok(())
    }

    pub fn trigger_event(&self, event: &str, data: serde_json::Value) -> Result<(), PluginError> {
        let plugins = self.plugins.lock().unwrap();

        for plugin in plugins.iter() {
            if !plugin.is_enabled() {
                continue;
            }

            let lua = &plugin.lua;
            if let Ok(handler) = lua.globals().get::<Function>(event) {
                let data_str = serde_json::to_string(&data).unwrap_or_default();
                if let Err(e) = handler.call::<_, ()>(data_str) {
                    eprintln!("Plugin {} event {} error: {}", plugin.name(), event, e);
                }
            }
        }

        Ok(())
    }

    pub fn get_plugin_list(&self) -> Vec<String> {
        let plugins = self.plugins.lock().unwrap();
        plugins.iter().map(|p| p.name().to_string()).collect()
    }

    pub fn enable_plugin(&self, name: &str) -> Result<(), PluginError> {
        let mut plugins = self.plugins.lock().unwrap();
        if let Some(plugin) = plugins.iter_mut().find(|p| p.name() == name) {
            plugin.set_enabled(true);
            Ok(())
        } else {
            Err(PluginError::NotFound(name.to_string()))
        }
    }

    pub fn disable_plugin(&self, name: &str) -> Result<(), PluginError> {
        let mut plugins = self.plugins.lock().unwrap();
        if let Some(plugin) = plugins.iter_mut().find(|p| p.name() == name) {
            plugin.set_enabled(false);
            Ok(())
        } else {
            Err(PluginError::NotFound(name.to_string()))
        }
    }
}
