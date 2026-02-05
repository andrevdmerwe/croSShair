use arboard::Clipboard;
use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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

/// Copy a Base64 encoded image to clipboard using arboard
#[tauri::command]
pub fn copy_image_to_clipboard(base64_data: String) -> Result<(), String> {
    // 1. Decode Base64
    let data = base64_data
        .strip_prefix("data:image/png;base64,")
        .unwrap_or(&base64_data);

    let bytes = general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    // 2. Load image to get raw pixels (RGBA)
    let img =
        load_from_memory(&bytes).map_err(|e| format!("Failed to load image from memory: {}", e))?;

    let rgba8 = img.into_rgba8();
    let (width, height) = rgba8.dimensions();
    let image_data = arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::Borrowed(rgba8.as_raw()),
    };

    // 3. Set to clipboard
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to initialize clipboard: {}", e))?;

    clipboard
        .set_image(image_data)
        .map_err(|e| format!("Failed to set image to clipboard: {}", e))?;

    Ok(())
}
