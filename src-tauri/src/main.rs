// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod feed;
mod rss;

use anyhow::Context;
use commands::*;
#[cfg(debug_assertions)]
use specta::collect_types;
use tauri::async_runtime::RwLock;
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
        .manage(RwLock::new(feed::Manager::new()))
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
        .invoke_handler(tauri::generate_handler![
            get_stories,
            get_subscriptions,
            add_feed,
            refresh,
        ])
        .run(tauri::generate_context!())
        .context("error while running tauri application")
}
