// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rss;

use anyhow::Context;
use tauri::async_runtime::RwLock;
use tauri_plugin_log::LogTarget;

#[tauri::command]
async fn get_items(url_list: tauri::State<'_, FeedList>) -> Result<Vec<rss::Item>, rss::Error> {
    let url_list = url_list.read().await;
    let feeds = rss::get_feeds(&url_list).await?;

    let mut items: Vec<rss::Item> = feeds.into_iter().flat_map(|feed| feed.items).collect();

    items.sort_by(|a, b| {
        use chrono::DateTime;

        let a = DateTime::parse_from_rfc2822(a.pub_date().unwrap()).unwrap();
        let b = DateTime::parse_from_rfc2822(b.pub_date().unwrap()).unwrap();

        b.cmp(&a)
    });

    Ok(items)
}

// if this isn't a result then this complains about lifetimes
#[tauri::command]
async fn get_subscriptions(url_list: tauri::State<'_, FeedList>) -> tauri::Result<Vec<String>> {
    Ok(url_list.read().await.clone())
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
        .invoke_handler(tauri::generate_handler![
            get_items,
            get_subscriptions,
            add_feed
        ])
        .run(tauri::generate_context!())
        .context("error while running tauri application")
}

type FeedList = RwLock<Vec<String>>;
