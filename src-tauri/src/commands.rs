use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[cfg(target_os = "macos")]
use crate::fs_macos::get_extended_attributes;

#[cfg(target_os = "windows")]
use crate::fs_windows::get_extended_attributes;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn get_extended_attributes(_path: &std::path::Path) -> anyhow::Result<(Vec<String>, std::collections::HashMap<String, String>)> {
    Ok((Vec::new(), std::collections::HashMap::new()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMeta {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub is_directory: bool,
    pub extension: Option<String>,
    pub tags: Vec<String>,
    pub attributes: std::collections::HashMap<String, String>,
}

impl FileMeta {
    pub fn new(path: &Path, metadata: &std::fs::Metadata) -> Result<Self> {
        let path_str = path.to_string_lossy().to_string();
        let name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let extension = path.extension()
            .map(|ext| ext.to_string_lossy().to_string());
        
        let modified = DateTime::from(metadata.modified()?);
        let created = DateTime::from(metadata.created()?);
        
        let (tags, attributes) = get_extended_attributes(path)?;
        
        Ok(FileMeta {
            path: path_str,
            name,
            size: metadata.len(),
            modified,
            created,
            is_directory: metadata.is_dir(),
            extension,
            tags,
            attributes,
        })
    }
}

#[tauri::command]
pub async fn scan_drive(path: String) -> Result<Vec<FileMeta>, String> {
    let scan_path = Path::new(&path);
    
    if !scan_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    let mut files = Vec::new();
    
    for entry in WalkDir::new(scan_path)
        .follow_links(false)
        .max_depth(3) // Limit depth to avoid excessive scanning
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(_) => continue, // Skip files we can't read
        };
        
        match FileMeta::new(entry.path(), &metadata) {
            Ok(file_meta) => files.push(file_meta),
            Err(_) => continue, // Skip files with errors
        }
    }
    
    Ok(files)
}

#[tauri::command]
pub async fn get_file_metadata(path: String) -> Result<FileMeta, String> {
    let file_path = Path::new(&path);
    
    if !file_path.exists() {
        return Err(format!("File does not exist: {}", path));
    }
    
    let metadata = std::fs::metadata(file_path)
        .map_err(|e| format!("Failed to get metadata: {}", e))?;
    
    FileMeta::new(file_path, &metadata)
        .map_err(|e| format!("Failed to create file metadata: {}", e))
}
