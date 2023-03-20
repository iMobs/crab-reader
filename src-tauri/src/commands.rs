use crate::{feed, rss};
use serde::Serialize;
use tauri::async_runtime::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
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
#[specta::specta]
pub async fn get_stories(
    manager: tauri::State<'_, RwLock<feed::Manager>>,
) -> CommandResult<Vec<feed::Story>> {
    Ok(manager.read().await.stories())
}

#[tauri::command]
#[specta::specta]
pub async fn get_subscriptions(
    manager: tauri::State<'_, RwLock<feed::Manager>>,
) -> CommandResult<Vec<feed::Subscription>> {
    Ok(manager.read().await.subscriptions())
}

#[tauri::command]
#[specta::specta]
pub async fn add_feed(
    url: String,
    manager: tauri::State<'_, RwLock<feed::Manager>>,
    window: tauri::Window,
) -> CommandResult<()> {
    let feed = rss::get_channel(&url).await?;
    manager.write().await.ingest(&feed)?;

    // TODO: find a way to emit without a dummy value
    window.emit("feed-refresh", 0)?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn refresh(
    manager: tauri::State<'_, RwLock<feed::Manager>>,
    window: tauri::Window,
) -> CommandResult<()> {
    for subscription in manager.read().await.subscriptions() {
        let channel = match rss::get_channel(subscription.url).await {
            Ok(channel) => channel,
            Err(e) => {
                log::error!("failed to fetch {}: {}", subscription.name, e);
                continue;
            }
        };

        if let Err(e) = manager.write().await.ingest(&channel) {
            log::error!("failed to ingest {}: {}", channel.title, e);
        }
    }

    // TODO: find a way to emit without a dummy value
    window.emit("feed-refresh", 0)?;

    Ok(())
}
