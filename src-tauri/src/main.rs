// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::Config;
use serde::{Deserialize, Serialize};
use std::fs;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb::{engine::local::File, opt::Strict};

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

#[derive(Debug, Serialize, Deserialize)]
struct Library {
    id: Option<Thing>,
    name: String,
    path: String,
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
async fn save_library(
    name: &str,
    path: &str,
    db: State<'_, Surreal<Db>>,
) -> Result<Library, String> {
    let l: Library = db
        .create("library")
        .content(Library {
            id: None,
            name: name.into(),
            path: path.into(),
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(l)
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
                let db = Surreal::new::<File>(("../stl.db", Strict)).await.unwrap();

                db.use_ns("stl-library").use_db("libraries").await.unwrap();

                app.manage(db);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            load_stl,
            scan_libraries,
            save_library
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
