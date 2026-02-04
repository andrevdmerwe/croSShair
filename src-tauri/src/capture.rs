use chrono::Local;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
use std::process::Command;

/// Result of dependency check
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyStatus {
    pub grim_available: bool,
    pub slurp_available: bool,
    pub wl_copy_available: bool,
    pub all_available: bool,
}

/// Result of a capture operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureResult {
    pub success: bool,
    pub file_path: Option<String>,
    pub error: Option<String>,
    pub cancelled: bool,
}

/// Check if a command exists in PATH
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check availability of required CLI tools
pub fn check_dependencies() -> DependencyStatus {
    let grim = command_exists("grim");
    let slurp = command_exists("slurp");
    let wl_copy = command_exists("wl-copy");

    DependencyStatus {
        grim_available: grim,
        slurp_available: slurp,
        wl_copy_available: wl_copy,
        all_available: grim && slurp && wl_copy,
    }
}

/// Generate timestamp-based filename
pub fn generate_filename() -> String {
    let now = Local::now();
    format!("{}.png", now.format("%Y%m%d_%H%M%S"))
}

/// Run slurp to get region selection
pub fn get_region_selection() -> Result<Option<String>, io::Error> {
    let output = Command::new("slurp").output()?;

    if output.status.success() {
        let geometry = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(Some(geometry))
    } else {
        // User cancelled (exit code 1 with no stderr usually means ESC pressed)
        if output.status.code() == Some(1) {
            Ok(None) // Cancelled
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::new(io::ErrorKind::Other, stderr.to_string()))
        }
    }
}

/// Capture a region using grim and save to file
pub fn capture_region(geometry: &str, output_path: &PathBuf) -> Result<(), io::Error> {
    let output = Command::new("grim")
        .arg("-g")
        .arg(geometry)
        .arg(output_path)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, stderr.to_string()))
    }
}

/// Capture a region and pipe directly to clipboard
pub fn capture_region_to_clipboard(geometry: &str) -> Result<(), io::Error> {
    // Use shell to pipe: grim -g "$geometry" - | wl-copy
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("grim -g '{}' - | wl-copy", geometry))
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, stderr.to_string()))
    }
}

/// Copy a file to clipboard using wl-copy
pub fn copy_file_to_clipboard(file_path: &PathBuf) -> Result<(), io::Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cat '{}' | wl-copy -t image/png",
            file_path.display()
        ))
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, stderr.to_string()))
    }
}

/// Full capture pipeline: slurp for region, grim to save, optionally copy to clipboard
pub fn capture_screenshot(screenshot_dir: &str, copy_to_clipboard: bool) -> CaptureResult {
    // Check dependencies first
    let deps = check_dependencies();
    if !deps.grim_available || !deps.slurp_available {
        return CaptureResult {
            success: false,
            file_path: None,
            error: Some("Missing required tools (grim/slurp). Install with: sudo pacman -S grim slurp wl-clipboard".to_string()),
            cancelled: false,
        };
    }

    // Get region selection
    let geometry = match get_region_selection() {
        Ok(Some(g)) => g,
        Ok(None) => {
            return CaptureResult {
                success: false,
                file_path: None,
                error: None,
                cancelled: true,
            };
        }
        Err(e) => {
            return CaptureResult {
                success: false,
                file_path: None,
                error: Some(format!("slurp failed: {}", e)),
                cancelled: false,
            };
        }
    };

    // Generate output path
    let filename = generate_filename();
    let output_path = PathBuf::from(screenshot_dir).join(&filename);

    // Ensure directory exists
    if let Err(e) = std::fs::create_dir_all(screenshot_dir) {
        return CaptureResult {
            success: false,
            file_path: None,
            error: Some(format!("Failed to create directory: {}", e)),
            cancelled: false,
        };
    }

    // Capture the screenshot
    if let Err(e) = capture_region(&geometry, &output_path) {
        return CaptureResult {
            success: false,
            file_path: None,
            error: Some(format!("grim failed: {}", e)),
            cancelled: false,
        };
    }

    // Optionally copy to clipboard
    if copy_to_clipboard && deps.wl_copy_available {
        if let Err(e) = copy_file_to_clipboard(&output_path) {
            // Non-fatal: log but continue
            eprintln!("Warning: Failed to copy to clipboard: {}", e);
        }
    }

    CaptureResult {
        success: true,
        file_path: Some(output_path.to_string_lossy().to_string()),
        error: None,
        cancelled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_filename() {
        let filename = generate_filename();
        assert!(filename.ends_with(".png"));
        assert!(filename.len() > 10);
    }
}
