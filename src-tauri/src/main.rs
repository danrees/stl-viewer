// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::Config;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::fs;
use tauri::{Manager, State};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    libraries: Vec<String>,
    extension: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct STLFile {
    path: String,
    name: String,
    tags: Vec<String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_stl(name: &str) -> Result<Vec<u8>, String> {
    println!("Path is: {}", name);
    let data = fs::read(name).map_err(|e| e.to_string())?;
    println!("Size of binary: {}", data.len());
    Ok(data)
}

#[tauri::command]
fn scan_libraries(config: State<AppConfig>) -> Result<(), String> {
    let dirs = &config.libraries;
    for dir in dirs {
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| {
                if let Some(ext) = f.path().extension() {
                    return ext == config.extension.as_str();
                }
                return false;
            })
        {
            println!("Found: {}", entry.path().as_os_str().to_string_lossy());
        }
    }
    Ok(())
}

fn main() {
    // TODO: create config directory if it doesn't already exist ... maybe create a default config
    tauri::Builder::default()
        .setup(|app| {
            let config_dir = tauri::api::path::app_config_dir(&app.config()).unwrap();
            let config = Config::builder()
                .add_source(config::File::with_name(
                    config_dir.join("stl-viewer-config").to_str().unwrap(),
                ))
                .build()
                .unwrap()
                .try_deserialize::<AppConfig>()
                .unwrap();
            app.manage(config);
            tauri::async_runtime::block_on(async move {
                let pool = SqlitePool::connect("sqlite://temp.db").await.unwrap();
                app.manage(pool);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, load_stl, scan_libraries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
