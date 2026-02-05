use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Save a Base64 encoded image to disk
#[tauri::command]
pub fn save_image(base64_data: String, file_path: String) -> Result<String, String> {
    // 1. Decode Base64
    // The data might come with a prefix "data:image/png;base64,", strip it if present
    let data = base64_data
        .strip_prefix("data:image/png;base64,")
        .unwrap_or(&base64_data);

    let bytes = general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    // 2. Write to file
    let path = PathBuf::from(&file_path);
    let mut file = File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(file_path)
}

/// Copy a Base64 encoded image to clipboard using wl-copy
#[tauri::command]
pub fn copy_image_to_clipboard(base64_data: String) -> Result<(), String> {
    // 1. Decode Base64
    let data = base64_data
        .strip_prefix("data:image/png;base64,")
        .unwrap_or(&base64_data);

    let bytes = general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    // 2. Pipe to wl-copy
    // echo <bytes> | wl-copy -t image/png
    // Note: echoing binary data in shell is tricky. Better to spawn process and write to stdin.

    let mut child = Command::new("wl-copy")
        .arg("-t")
        .arg("image/png")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn wl-copy: {}. Is it installed?", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(&bytes)
            .map_err(|e| format!("Failed to write to wl-copy stdin: {}", e))?;
    }

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for wl-copy: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("wl-copy failed (non-zero exit code)".to_string())
    }
}
