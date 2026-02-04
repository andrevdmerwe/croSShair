use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub screenshot_directory: String,
    pub copy_after_capture: bool,
    pub default_color: String,
    pub default_stroke_width: StrokeWidth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StrokeWidth {
    Thin,
    Medium,
    Thick,
}

impl Default for StrokeWidth {
    fn default() -> Self {
        StrokeWidth::Medium
    }
}

/// Get the XDG-compliant config directory path
fn get_config_dir() -> PathBuf {
    // Check XDG_CONFIG_HOME first, fallback to ~/.config
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config).join("croSShair")
    } else if let Some(home) = dirs::home_dir() {
        home.join(".config").join("croSShair")
    } else {
        PathBuf::from(".config").join("croSShair")
    }
}

/// Get the config file path
fn get_config_path() -> PathBuf {
    get_config_dir().join("config.json")
}

/// Get default screenshot directory
fn get_default_screenshot_dir() -> String {
    if let Some(home) = dirs::home_dir() {
        home.join("Pictures")
            .join("Screenshots")
            .to_string_lossy()
            .to_string()
    } else {
        "~/Pictures/Screenshots".to_string()
    }
}

/// Get default configuration
pub fn get_default_config() -> AppConfig {
    AppConfig {
        screenshot_directory: get_default_screenshot_dir(),
        copy_after_capture: false,
        default_color: "#ff0000".to_string(), // Red
        default_stroke_width: StrokeWidth::Medium,
    }
}

/// Load configuration from file, creating defaults if not exists
pub fn get_config() -> io::Result<AppConfig> {
    let config_path = get_config_path();

    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        let config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    } else {
        // Create default config and save it
        let default_config = get_default_config();
        save_config(&default_config)?;
        Ok(default_config)
    }
}

/// Save configuration to file
pub fn save_config(config: &AppConfig) -> io::Result<()> {
    let config_dir = get_config_dir();
    let config_path = get_config_path();

    // Ensure config directory exists
    fs::create_dir_all(&config_dir)?;

    // Ensure screenshot directory exists
    let screenshot_dir = PathBuf::from(&config.screenshot_directory);
    if !screenshot_dir.exists() {
        fs::create_dir_all(&screenshot_dir)?;
    }

    // Write config as pretty JSON
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(&config_path, content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = get_default_config();
        assert!(!config.copy_after_capture);
        assert_eq!(config.default_color, "#ff0000");
    }
}
