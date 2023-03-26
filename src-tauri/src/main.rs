// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Error;
use std::fs;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_stl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
