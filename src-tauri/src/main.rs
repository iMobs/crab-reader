// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod feed;
mod rss;

use anyhow::Context;
use commands::*;
#[cfg(debug_assertions)]
use specta::collect_types;
use tauri::{async_runtime::RwLock, Manager};
use tauri_plugin_log::LogTarget;
#[cfg(debug_assertions)]
use tauri_specta::ts;

fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![get_stories, get_subscriptions, add_feed, refresh],
        "../src/lib/bindings.ts",
    )?;

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    LogTarget::Stdout,
                    // LogTarget::LogDir,
                    LogTarget::Webview,
                ])
                .level(if cfg!(debug_assertions) {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .filter(|metadata| metadata.target().starts_with("crab_reader"))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_stories,
            get_subscriptions,
            add_feed,
            refresh,
        ])
        .setup(setup_app)
        .run(tauri::generate_context!())
        .context("error while running tauri application")
}

fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = app.path_resolver().app_data_dir().expect("no app data dir");
    let manager = feed::Manager::load(data_dir);
    app.manage(RwLock::new(manager));

    Ok(())
}
