// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rss;

use anyhow::Context;
use tauri_plugin_log::LogTarget;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    log::debug!("Hello from the backend!");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_feeds(urls: Vec<String>) -> Vec<String> {
    rss::get_feeds(&urls).await.unwrap()
}

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    LogTarget::Stdout,
                    // LogTarget::LogDir,
                    LogTarget::Webview,
                ])
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet, get_feeds])
        .run(tauri::generate_context!())
        .context("error while running tauri application")
}
