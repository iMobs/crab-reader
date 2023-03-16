// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod feed;
mod rss;

use anyhow::Context;
use serde::Serialize;
use tauri::async_runtime::RwLock;
use tauri_plugin_log::LogTarget;

#[derive(Debug, thiserror::Error)]
enum CommandError {
    #[error(transparent)]
    Feed(#[from] feed::Error),

    #[error(transparent)]
    Rss(#[from] rss::Error),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type CommandResult<T> = Result<T, CommandError>;

#[tauri::command]
async fn get_stories(
    manager: tauri::State<'_, RwLock<feed::Manager>>,
) -> CommandResult<Vec<feed::Story>> {
    let manager = manager.read().await;

    Ok(manager.stories())
}

// if this isn't a result then this complains about lifetimes
#[tauri::command]
async fn get_subscriptions(
    manager: tauri::State<'_, RwLock<feed::Manager>>,
) -> CommandResult<Vec<feed::Subscription>> {
    let manager = manager.read().await;

    Ok(manager.subscriptions())
}

#[tauri::command]
async fn add_feed(
    url: String,
    manager: tauri::State<'_, RwLock<feed::Manager>>,
    window: tauri::Window,
) -> CommandResult<()> {
    let feed = rss::get_channel(&url).await?;
    manager.write().await.ingest(feed)?;

    // TODO: find a way to emit without a dummy value
    window.emit("feed-refresh", 0)?;

    Ok(())
}

#[tauri::command]
async fn refresh(manager: tauri::State<'_, RwLock<feed::Manager>>) -> CommandResult<()> {
    let mut manager = manager.write().await;

    let urls: Vec<String> = manager.subscriptions().into_iter().map(|s| s.url).collect();

    let channels = rss::get_channels(&urls).await?;

    for channel in channels {
        manager.ingest(channel)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
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
