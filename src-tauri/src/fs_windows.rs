use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use anyhow::{Result, anyhow};
use winapi::um::fileapi::{GetFileAttributesW, INVALID_FILE_ATTRIBUTES};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_COMPRESSED, FILE_ATTRIBUTE_DIRECTORY,
    FILE_ATTRIBUTE_ENCRYPTED, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_NORMAL,
    FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM, FILE_ATTRIBUTE_TEMPORARY,
};
use winapi::um::shellapi::{SHGetFileInfoW, SHFILEINFOW, SHGFI_TYPENAME};

/// Get extended attributes and Windows file properties
pub fn get_extended_attributes(path: &Path) -> Result<(Vec<String>, HashMap<String, String>)> {
    let mut attributes = HashMap::new();
    let tags = Vec::new(); // Windows doesn't have native tags like macOS
    
    // Convert path to wide string for Windows API
    let path_wide: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(Some(0))
        .collect();
    
    // Get file attributes
    let file_attrs = unsafe { GetFileAttributesW(path_wide.as_ptr()) };
    
    if file_attrs == INVALID_FILE_ATTRIBUTES {
        return Ok((tags, attributes));
    }
    
    // Parse file attributes
    let mut attr_strings = Vec::new();
    
    if file_attrs & FILE_ATTRIBUTE_ARCHIVE != 0 {
        attr_strings.push("Archive");
    }
    if file_attrs & FILE_ATTRIBUTE_COMPRESSED != 0 {
        attr_strings.push("Compressed");
    }
    if file_attrs & FILE_ATTRIBUTE_DIRECTORY != 0 {
        attr_strings.push("Directory");
    }
    if file_attrs & FILE_ATTRIBUTE_ENCRYPTED != 0 {
        attr_strings.push("Encrypted");
    }
    if file_attrs & FILE_ATTRIBUTE_HIDDEN != 0 {
        attr_strings.push("Hidden");
    }
    if file_attrs & FILE_ATTRIBUTE_NORMAL != 0 {
        attr_strings.push("Normal");
    }
    if file_attrs & FILE_ATTRIBUTE_READONLY != 0 {
        attr_strings.push("ReadOnly");
    }
    if file_attrs & FILE_ATTRIBUTE_SYSTEM != 0 {
        attr_strings.push("System");
    }
    if file_attrs & FILE_ATTRIBUTE_TEMPORARY != 0 {
        attr_strings.push("Temporary");
    }
    
    attributes.insert("windows_attributes".to_string(), attr_strings.join(", "));
    
    // Get file type information
    if let Ok(file_type) = get_file_type(path) {
        attributes.insert("file_type".to_string(), file_type);
    }
    
    // Get file description (if available)
    if let Ok(description) = get_file_description(path) {
        attributes.insert("description".to_string(), description);
    }
    
    Ok((tags, attributes))
}

fn get_file_type(path: &Path) -> Result<String> {
    let path_wide: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(Some(0))
        .collect();
    
    let mut file_info: SHFILEINFOW = unsafe { std::mem::zeroed() };
    
    let result = unsafe {
        SHGetFileInfoW(
            path_wide.as_ptr(),
            0,
            &mut file_info,
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_TYPENAME,
        )
    };
    
    if result == 0 {
        return Err(anyhow!("Failed to get file type"));
    }
    
    // Convert wide string to Rust string
    let type_name = unsafe {
        let len = file_info.szTypeName.iter().position(|&x| x == 0).unwrap_or(0);
        String::from_utf16_lossy(&file_info.szTypeName[..len])
    };
    
    Ok(type_name)
}

fn get_file_description(path: &Path) -> Result<String> {
    // This is a simplified implementation
    // In a real application, you might want to use more sophisticated methods
    // to extract file descriptions from version resources, etc.
    
    if let Some(extension) = path.extension() {
        let ext_str = extension.to_string_lossy().to_lowercase();
        let description = match ext_str.as_str() {
            "exe" => "Application",
            "dll" => "Dynamic Link Library",
            "sys" => "System File",
            "bat" => "Batch File",
            "cmd" => "Command Script",
            "ps1" => "PowerShell Script",
            "vbs" => "VBScript File",
            "reg" => "Registry File",
            "msi" => "Windows Installer Package",
            "zip" => "ZIP Archive",
            "rar" => "RAR Archive",
            "7z" => "7-Zip Archive",
            "pdf" => "PDF Document",
            "doc" | "docx" => "Microsoft Word Document",
            "xls" | "xlsx" => "Microsoft Excel Spreadsheet",
            "ppt" | "pptx" => "Microsoft PowerPoint Presentation",
            "txt" => "Text Document",
            "rtf" => "Rich Text Format",
            "html" | "htm" => "HTML Document",
            "css" => "Cascading Style Sheet",
            "js" => "JavaScript File",
            "json" => "JSON File",
            "xml" => "XML Document",
            "csv" => "Comma Separated Values",
            "jpg" | "jpeg" => "JPEG Image",
            "png" => "PNG Image",
            "gif" => "GIF Image",
            "bmp" => "Bitmap Image",
            "ico" => "Icon File",
            "mp3" => "MP3 Audio",
            "wav" => "WAV Audio",
            "flac" => "FLAC Audio",
            "mp4" => "MP4 Video",
            "avi" => "AVI Video",
            "mkv" => "Matroska Video",
            "mov" => "QuickTime Movie",
            _ => "File",
        };
        Ok(description.to_string())
    } else {
        Ok("File".to_string())
    }
}

/// Check if a file is hidden on Windows
pub fn is_hidden_file(path: &Path) -> bool {
    let path_wide: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(Some(0))
        .collect();
    
    let file_attrs = unsafe { GetFileAttributesW(path_wide.as_ptr()) };
    
    if file_attrs == INVALID_FILE_ATTRIBUTES {
        return false;
    }
    
    file_attrs & FILE_ATTRIBUTE_HIDDEN != 0
}

/// Get Windows-specific file dates
pub fn get_file_dates(path: &Path) -> Result<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> {
    let metadata = std::fs::metadata(path)?;
    let created = chrono::DateTime::from(metadata.created()?);
    let modified = chrono::DateTime::from(metadata.modified()?);
    Ok((created, modified))
}

/// Check if a file has alternate data streams (ADS)
pub fn has_alternate_data_streams(path: &Path) -> bool {
    // This is a simplified check
    // In a real implementation, you would use FindFirstStreamW/FindNextStreamW
    // to enumerate all streams
    false
}

/// Get file security information
pub fn get_file_security_info(path: &Path) -> Result<HashMap<String, String>> {
    let mut security_info = HashMap::new();
    
    // This is a placeholder for more sophisticated security information
    // In a real implementation, you would use GetFileSecurity and related APIs
    
    if is_hidden_file(path) {
        security_info.insert("hidden".to_string(), "true".to_string());
    }
    
    let metadata = std::fs::metadata(path)?;
    if metadata.permissions().readonly() {
        security_info.insert("readonly".to_string(), "true".to_string());
    }
    
    Ok(security_info)
}
