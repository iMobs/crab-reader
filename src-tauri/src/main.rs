// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rss;

use anyhow::Context;
use tauri::async_runtime::RwLock;
use tauri_plugin_log::LogTarget;

#[tauri::command]
async fn get_feeds(url_list: tauri::State<'_, FeedList>) -> Result<Vec<rss::Channel>, rss::Error> {
    let url_list = url_list.read().await;
    rss::get_feeds(&url_list).await
}

#[tauri::command]
async fn add_feed(
    url: String,
    url_list: tauri::State<'_, FeedList>,
    window: tauri::Window,
) -> tauri::Result<()> {
    // TODO: cache the feed and bubble up errors
    if let Ok(_feed) = rss::get_feed(&url).await {
        let mut url_list = url_list.write().await;
        url_list.push(url);
        log::debug!("url_list: {:?}", url_list);
        window.emit("feed-refresh", url_list.clone())?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let feed_list: Vec<String> = Vec::new();

    tauri::Builder::default()
        .manage(RwLock::new(feed_list))
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
        .invoke_handler(tauri::generate_handler![get_feeds, add_feed])
        .run(tauri::generate_context!())
        .context("error while running tauri application")
}

type FeedList = RwLock<Vec<String>>;
