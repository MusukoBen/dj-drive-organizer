use gloo_events::EventListener;
use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use js_sys::Promise;
use web_sys::console;

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub current_file: String,
    pub files_processed: u64,
    pub total_files: Option<u64>,
    pub percentage: Option<f64>,
}

// Drive context for sharing state across components
#[derive(Debug, Clone)]
pub struct DriveContextData {
    pub current_files: Vec<FileMeta>,
    pub scan_progress: Option<ScanProgress>,
    pub is_scanning: bool,
    pub error: Option<String>,
}

impl Default for DriveContextData {
    fn default() -> Self {
        Self {
            current_files: Vec::new(),
            scan_progress: None,
            is_scanning: false,
            error: None,
        }
    }
}

pub type DriveContext = UseStateHandle<DriveContextData>;

// Context provider component
#[derive(Properties, PartialEq)]
pub struct DriveContextProviderProps {
    pub children: Children,
}

#[function_component]
pub fn DriveContextProvider(props: &DriveContextProviderProps) -> Html {
    let drive_context = use_state(DriveContextData::default);
    
    html! {
        <ContextProvider<DriveContext> context={drive_context}>
            {props.children.clone()}
        </ContextProvider<DriveContext>>
    }
}

// Hook for consuming the drive context
#[hook]
pub fn use_drive_context() -> DriveContext {
    use_context::<DriveContext>().expect("DriveContext not found")
}

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
    
    spawn_local(async move {
        let _ = wasm_bindgen_futures::JsFuture::from(tauri_emit(event_name, &js_payload)).await;
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

// Utility functions for the drive context
impl DriveContextData {
    pub fn set_files(&mut self, files: Vec<FileMeta>) {
        self.current_files = files;
        self.error = None;
    }
    
    pub fn set_scanning(&mut self, is_scanning: bool) {
        self.is_scanning = is_scanning;
        if !is_scanning {
            self.scan_progress = None;
        }
    }
    
    pub fn set_scan_progress(&mut self, progress: ScanProgress) {
        self.scan_progress = Some(progress);
    }
    
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.is_scanning = false;
        self.scan_progress = None;
    }
    
    pub fn clear_error(&mut self) {
        self.error = None;
    }
    
    pub fn get_audio_files(&self) -> Vec<&FileMeta> {
        self.current_files.iter().filter(|f| f.is_audio).collect()
    }
    
    pub fn get_files_by_extension(&self, extension: &str) -> Vec<&FileMeta> {
        self.current_files
            .iter()
            .filter(|f| f.path.to_lowercase().ends_with(&extension.to_lowercase()))
            .collect()
    }
}

// High-level convenience functions that integrate with the context
pub fn scan_drive_with_context(path: String, context: &DriveContext) {
    let context_clone = context.clone();
    
    spawn_local(async move {
        // Set scanning state
        {
            let mut data = (*context_clone).clone();
            data.set_scanning(true);
            data.clear_error();
            context_clone.set(data);
        }
        
        // Listen to progress updates
        let progress_context = context_clone.clone();
        let _ = listen_to_scan_progress(move |progress| {
            let mut data = (*progress_context).clone();
            data.set_scan_progress(progress);
            progress_context.set(data);
        });
        
        // Perform the scan
        match invoke_scan_drive(path).await {
            Ok(files) => {
                let mut data = (*context_clone).clone();
                data.set_files(files);
                data.set_scanning(false);
                context_clone.set(data);
            }
            Err(e) => {
                let mut data = (*context_clone).clone();
                data.set_error(format!("Scan failed: {}", e));
                context_clone.set(data);
            }
        }
    });
}

pub fn organize_files_with_context(
    source_path: String,
    target_path: String,
    options: OrganizeOptions,
    context: &DriveContext,
) {
    let context_clone = context.clone();
    
    spawn_local(async move {
        match invoke_organize_files(source_path, target_path, options).await {
            Ok(result) => {
                // You might want to emit a success event or update the context
                // depending on your application's needs
                web_sys::console::log_1(&format!("Organization completed: {:?}", result).into());
            }
            Err(e) => {
                let mut data = (*context_clone).clone();
                data.set_error(format!("Organization failed: {}", e));
                context_clone.set(data);
            }
        }
    });
}
