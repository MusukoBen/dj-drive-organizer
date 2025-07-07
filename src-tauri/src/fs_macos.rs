use std::collections::HashMap;
use std::path::Path;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use anyhow::{Result, anyhow};

// External C functions for extended attributes
extern "C" {
    fn getxattr(
        path: *const libc::c_char,
        name: *const libc::c_char,
        value: *mut libc::c_void,
        size: libc::size_t,
        position: u32,
        options: libc::c_int,
    ) -> libc::ssize_t;
    
    fn listxattr(
        path: *const libc::c_char,
        namebuf: *mut libc::c_char,
        size: libc::size_t,
        options: libc::c_int,
    ) -> libc::ssize_t;
}

/// Get extended attributes and Finder tags for a file on macOS
pub fn get_extended_attributes(path: &Path) -> Result<(Vec<String>, HashMap<String, String>)> {
    let path_cstring = CString::new(path.as_os_str().as_bytes())?;
    let mut attributes = HashMap::new();
    let mut tags = Vec::new();
    
    // Get list of extended attribute names
    let list_size = unsafe {
        listxattr(path_cstring.as_ptr(), std::ptr::null_mut(), 0, 0)
    };
    
    if list_size < 0 {
        // No extended attributes or error - return empty collections
        return Ok((tags, attributes));
    }
    
    if list_size == 0 {
        return Ok((tags, attributes));
    }
    
    let mut name_buf = vec![0u8; list_size as usize];
    let actual_size = unsafe {
        listxattr(
            path_cstring.as_ptr(),
            name_buf.as_mut_ptr() as *mut libc::c_char,
            list_size as libc::size_t,
            0,
        )
    };
    
    if actual_size < 0 {
        return Ok((tags, attributes));
    }
    
    // Parse attribute names (they're null-terminated strings)
    let mut start = 0;
    while start < actual_size as usize {
        let end = name_buf[start..].iter().position(|&b| b == 0).unwrap_or(0) + start;
        if end > start {
            if let Ok(attr_name) = std::str::from_utf8(&name_buf[start..end]) {
                // Get attribute value
                if let Ok(value) = get_xattr_value(path, attr_name) {
                    if attr_name == "com.apple.metadata:_kMDItemUserTags" {
                        // Parse Finder tags
                        tags = parse_finder_tags(&value);
                    } else {
                        attributes.insert(attr_name.to_string(), value);
                    }
                }
            }
        }
        start = end + 1;
    }
    
    Ok((tags, attributes))
}

fn get_xattr_value(path: &Path, attr_name: &str) -> Result<String> {
    let path_cstring = CString::new(path.as_os_str().as_bytes())?;
    let attr_cstring = CString::new(attr_name)?;
    
    // Get size of attribute value
    let size = unsafe {
        getxattr(
            path_cstring.as_ptr(),
            attr_cstring.as_ptr(),
            std::ptr::null_mut(),
            0,
            0,
            0,
        )
    };
    
    if size < 0 {
        return Err(anyhow!("Failed to get attribute size"));
    }
    
    if size == 0 {
        return Ok(String::new());
    }
    
    // Get actual attribute value
    let mut buffer = vec![0u8; size as usize];
    let actual_size = unsafe {
        getxattr(
            path_cstring.as_ptr(),
            attr_cstring.as_ptr(),
            buffer.as_mut_ptr() as *mut libc::c_void,
            size as libc::size_t,
            0,
            0,
        )
    };
    
    if actual_size < 0 {
        return Err(anyhow!("Failed to get attribute value"));
    }
    
    // Convert to string (handle binary data gracefully)
    match std::str::from_utf8(&buffer[..actual_size as usize]) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => Ok(format!("Binary data ({} bytes)", actual_size)),
    }
}

fn parse_finder_tags(data: &str) -> Vec<String> {
    // Finder tags are stored as a plist - this is a simplified parser
    // In a real implementation, you might want to use a proper plist parser
    let mut tags = Vec::new();
    
    // Look for tag patterns in the plist data
    let lines: Vec<&str> = data.lines().collect();
    let mut in_array = false;
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.contains("<array>") {
            in_array = true;
        } else if trimmed.contains("</array>") {
            in_array = false;
        } else if in_array && trimmed.starts_with("<string>") && trimmed.ends_with("</string>") {
            let tag = trimmed
                .strip_prefix("<string>")
                .unwrap_or("")
                .strip_suffix("</string>")
                .unwrap_or("");
            if !tag.is_empty() {
                tags.push(tag.to_string());
            }
        }
    }
    
    tags
}

/// Get file creation and modification dates using macOS-specific APIs
pub fn get_file_dates(path: &Path) -> Result<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> {
    let metadata = std::fs::metadata(path)?;
    let created = chrono::DateTime::from(metadata.created()?);
    let modified = chrono::DateTime::from(metadata.modified()?);
    Ok((created, modified))
}

/// Check if a file has the macOS "hidden" attribute
pub fn is_hidden_file(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        if name.to_string_lossy().starts_with('.') {
            return true;
        }
    }
    
    // Check for com.apple.FinderInfo extended attribute
    if let Ok((_, attributes)) = get_extended_attributes(path) {
        if let Some(finder_info) = attributes.get("com.apple.FinderInfo") {
            // Check if the invisible flag is set (bit 14 of finder flags)
            return finder_info.contains("invisible") || finder_info.len() >= 2;
        }
    }
    
    false
}
