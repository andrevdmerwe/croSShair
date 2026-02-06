use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

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

/// Copy a Base64 encoded image to clipboard
/// Uses wl-copy for Wayland or xclip for X11
#[tauri::command]
pub fn copy_image_to_clipboard(base64_data: String) -> Result<(), String> {
    // 1. Decode Base64
    let data = base64_data
        .strip_prefix("data:image/png;base64,")
        .unwrap_or(&base64_data);

    let bytes = general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    // 2. Write to a temp file
    let temp_path = std::env::temp_dir().join("crosshair_clipboard.png");
    let mut file =
        File::create(&temp_path).map_err(|e| format!("Failed to create temp file: {}", e))?;
    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;
    drop(file); // Ensure file is closed before reading

    // 3. Try wl-copy first (Wayland), then xclip (X11)
    // wl-copy stays alive to serve clipboard - spawn and don't wait
    let png_data =
        std::fs::read(&temp_path).map_err(|e| format!("Failed to read temp file: {}", e))?;

    let wl_result = Command::new("/usr/bin/wl-copy")
        .arg("--type")
        .arg("image/png")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            // Write data to stdin and drop it to signal EOF
            if let Some(ref mut stdin) = child.stdin.take() {
                stdin.write_all(&png_data)?;
            }
            // Don't wait - wl-copy runs in background to serve clipboard
            Ok(())
        });

    match wl_result {
        Ok(()) => {
            // wl-copy started successfully, cleanup temp file
            let _ = std::fs::remove_file(&temp_path);
            return Ok(());
        }
        Err(e) => {
            // wl-copy couldn't be started
            eprintln!("wl-copy error: {}", e);
        }
    }

    // wl-copy failed, try xclip
    let xclip_result = Command::new("/usr/bin/xclip")
        .arg("-selection")
        .arg("clipboard")
        .arg("-t")
        .arg("image/png")
        .arg("-i")
        .arg(&temp_path)
        .output();

    match xclip_result {
        Ok(output) if output.status.success() => {
            let _ = std::fs::remove_file(&temp_path);
            return Ok(());
        }
        Ok(output) => {
            let _ = std::fs::remove_file(&temp_path);
            return Err(format!(
                "xclip failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Err(e) => {
            let _ = std::fs::remove_file(&temp_path);
            return Err(format!(
                "Clipboard copy failed. Please install wl-copy (Wayland) or xclip (X11): {}",
                e
            ));
        }
    }
}
