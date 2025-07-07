use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use js_sys::Promise;

// File metadata structure that matches the backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileMeta {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub modified: String,
    pub is_audio: bool,
    pub audio_info: Option<AudioInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioInfo {
    pub duration: Option<f64>,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub bpm: Option<f64>,
    pub key: Option<String>,
}

// Drive scan progress event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanProgress {
    pub current_file: String,
    pub files_processed: u64,
    pub total_files: Option<u64>,
    pub percentage: Option<f64>,
}

// Legacy context code removed - using minimal_drive_hooks instead

// Wasm-bindgen bindings for Tauri API
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = "invoke")]
    fn tauri_invoke(command: &str, args: &JsValue) -> Promise;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = "listen")]
    fn tauri_listen(event: &str, callback: &js_sys::Function) -> Promise;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = "emit")]
    fn tauri_emit(event: &str, payload: &JsValue) -> Promise;
}

// Tauri command invocation functions
pub async fn invoke_scan_drive(path: String) -> anyhow::Result<Vec<FileMeta>> {
    let args = serde_json::json!({ "path": path });
    let js_args = serde_wasm_bindgen::to_value(&args).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))?;
    
    let promise = tauri_invoke("scan_drive", &js_args);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| anyhow::anyhow!("Tauri invoke error: {:?}", e))?;
    
    serde_wasm_bindgen::from_value(result).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
}

pub async fn invoke_organize_files(
    source_path: String,
    target_path: String,
    options: OrganizeOptions,
) -> anyhow::Result<OrganizeResult> {
    let args = serde_json::json!({
        "source_path": source_path,
        "target_path": target_path,
        "options": options
    });
    let js_args = serde_wasm_bindgen::to_value(&args).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))?;
    
    let promise = tauri_invoke("organize_files", &js_args);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| anyhow::anyhow!("Tauri invoke error: {:?}", e))?;
    
    serde_wasm_bindgen::from_value(result).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
}

pub async fn invoke_analyze_audio(file_path: String) -> anyhow::Result<AudioInfo> {
    let args = serde_json::json!({ "file_path": file_path });
    let js_args = serde_wasm_bindgen::to_value(&args).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))?;
    
    let promise = tauri_invoke("analyze_audio", &js_args);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| anyhow::anyhow!("Tauri invoke error: {:?}", e))?;
    
    serde_wasm_bindgen::from_value(result).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
}

pub async fn invoke_get_drives() -> anyhow::Result<Vec<DriveInfo>> {
    let args = serde_json::json!({});
    let js_args = serde_wasm_bindgen::to_value(&args).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))?;
    
    let promise = tauri_invoke("get_drives", &js_args);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| anyhow::anyhow!("Tauri invoke error: {:?}", e))?;
    
    serde_wasm_bindgen::from_value(result).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
}

pub async fn invoke_create_backup(
    source_path: String,
    backup_path: String,
) -> anyhow::Result<BackupResult> {
    let args = serde_json::json!({
        "source_path": source_path,
        "backup_path": backup_path
    });
    let js_args = serde_wasm_bindgen::to_value(&args).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))?;
    
    let promise = tauri_invoke("create_backup", &js_args);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| anyhow::anyhow!("Tauri invoke error: {:?}", e))?;
    
    serde_wasm_bindgen::from_value(result).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
}

// Additional data structures for commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizeOptions {
    pub by_artist: bool,
    pub by_album: bool,
    pub by_genre: bool,
    pub by_year: bool,
    pub by_bpm: bool,
    pub by_key: bool,
    pub create_folders: bool,
    pub move_files: bool, // true for move, false for copy
    pub preserve_structure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizeResult {
    pub files_processed: u64,
    pub files_moved: u64,
    pub files_copied: u64,
    pub errors: Vec<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveInfo {
    pub name: String,
    pub path: String,
    pub total_space: u64,
    pub free_space: u64,
    pub drive_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupResult {
    pub files_backed_up: u64,
    pub total_size: u64,
    pub duration_ms: u64,
    pub backup_path: String,
}

// Event handling functions
pub fn emit_event(event_name: &str, payload: &impl Serialize) -> Result<(), JsValue> {
    let js_payload = serde_wasm_bindgen::to_value(payload).map_err(|_| JsValue::from_str("Serialization error"))?;
    let event_name = event_name.to_string();
    
    spawn_local(async move {
        let _ = wasm_bindgen_futures::JsFuture::from(tauri_emit(&event_name, &js_payload)).await;
    });
    
    Ok(())
}

pub fn listen_to_event<F, T>(event_name: &str, callback: F) -> Result<(), JsValue>
where
    F: Fn(T) + 'static,
    T: for<'de> Deserialize<'de> + 'static,
{
    let event_name = event_name.to_string();
    let callback = Rc::new(callback);
    
    spawn_local(async move {
        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(payload) = serde_wasm_bindgen::from_value::<T>(event) {
                callback(payload);
            }
        }) as Box<dyn Fn(JsValue)>);
        
        let js_callback = closure.as_ref().unchecked_ref::<js_sys::Function>();
        let _ = wasm_bindgen_futures::JsFuture::from(tauri_listen(&event_name, js_callback)).await;
        
        // Keep the closure alive - in a real app you'd want to store this for cleanup
        closure.forget();
    });
    
    Ok(())
}

// Helper function to listen to scan progress events
pub fn listen_to_scan_progress<F>(callback: F) -> Result<(), JsValue>
where
    F: Fn(ScanProgress) + 'static,
{
    listen_to_event("scan_progress", callback)
}

// Helper function to listen to organization progress events
pub fn listen_to_organize_progress<F>(callback: F) -> Result<(), JsValue>
where
    F: Fn(OrganizeProgress) + 'static,
{
    listen_to_event("organize_progress", callback)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizeProgress {
    pub current_file: String,
    pub files_processed: u64,
    pub total_files: u64,
    pub percentage: f64,
    pub current_operation: String, // "scanning", "organizing", "copying", "moving"
}


// Legacy drive state hooks removed - using minimal_drive_hooks instead
