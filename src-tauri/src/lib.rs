mod capture;
mod config;
mod export;

use capture::{capture_screenshot, check_dependencies, CaptureResult, DependencyStatus};
use config::{get_config, get_default_config, save_config, AppConfig};
use std::path::PathBuf;
use tauri::{Emitter, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_config() -> Result<AppConfig, String> {
    get_config().map_err(|e| e.to_string())
}

#[tauri::command]
fn update_config(config: AppConfig) -> Result<(), String> {
    save_config(&config).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_defaults() -> AppConfig {
    get_default_config()
}

#[tauri::command]
fn check_cli_dependencies() -> DependencyStatus {
    check_dependencies()
}

#[tauri::command]
fn take_screenshot(save_path: Option<String>) -> CaptureResult {
    let config = match get_config() {
        Ok(c) => c,
        Err(e) => {
            return CaptureResult {
                success: false,
                file_path: None,
                error: Some(format!("Failed to load config: {}", e)),
                cancelled: false,
            };
        }
    };

    // Use provided path or config directory
    let directory = save_path
        .filter(|p| !p.is_empty())
        .unwrap_or(config.screenshot_directory);

    // Log the intended path
    println!("Taking screenshot. Directory: {}", directory);

    let result = capture_screenshot(&directory, config.copy_after_capture);

    if let Some(ref path) = result.file_path {
        println!("Screenshot saved to: {}", path);
        // Verify file exists
        if std::path::Path::new(path).exists() {
            println!("File verification: EXISTS");
            // Check size
            if let Ok(metadata) = std::fs::metadata(path) {
                println!("File size: {} bytes", metadata.len());
            }
        } else {
            println!("File verification: MISSING!");
        }
    } else if let Some(ref err) = result.error {
        println!("Capture failed: {}", err);
    }

    result
}

#[tauri::command]
fn check_file_exists(path: String) -> bool {
    let exists = std::path::Path::new(&path).exists();
    println!("Frontend requested check for '{}': {}", path, exists);
    exists
}

use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::Read;

#[tauri::command]
fn read_image_base64(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let mut file = File::open(&path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    let encoded = general_purpose::STANDARD.encode(&buffer);
    Ok(format!("data:image/png;base64,{}", encoded))
}

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // System Tray Setup
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(
                    tauri::image::Image::from_bytes(include_bytes!("../icons/tray-icon.png"))
                        .expect("tray icon not found"),
                )
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            // Handle arguments from secondary instance
            // This will be used for CLI capture-region command
            if args.len() > 1 && args[1] == "capture-region" {
                // Emit event to frontend to trigger capture
                let _ = app.emit("capture-region-request", ());
            }
            // Focus the main window
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            update_config,
            get_defaults,
            check_cli_dependencies,
            take_screenshot,
            export::save_image,
            export::copy_image_to_clipboard,
            check_file_exists,
            read_image_base64
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
