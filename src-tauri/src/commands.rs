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
pub async fn add_subscription(
    url: &str,
    manager: tauri::State<'_, RwLock<feed::Manager>>,
    window: tauri::Window,
) -> CommandResult<()> {
    let mut manager = manager.write().await;
    let channel = rss::get_channel(url).await?;
    manager.ingest(url, &channel)?;
    manager.save()?;

    feed_refresh(&manager, &window).await
}

#[tauri::command]
#[specta::specta]
pub async fn remove_subscription(
    name: &str,
    manager: tauri::State<'_, RwLock<feed::Manager>>,
    window: tauri::Window,
) -> CommandResult<()> {
    let mut manager = manager.write().await;
    manager.remove_subscription(name);
    manager.save()?;

    feed_refresh(&manager, &window).await
}

#[tauri::command]
#[specta::specta]
pub async fn refresh(
    manager: tauri::State<'_, RwLock<feed::Manager>>,
    window: tauri::Window,
) -> CommandResult<()> {
    let mut manager = manager.write().await;
    log::debug!("refreshing subscriptions");

    for subscription in manager.subscriptions() {
        log::debug!("fetching {}", subscription.url);
        let channel = match rss::get_channel(&subscription.url).await {
            Ok(channel) => channel,
            Err(e) => {
                log::error!("failed to fetch {}: {}", subscription.name, e);
                continue;
            }
        };

        if let Err(e) = manager.ingest(&subscription.url, &channel) {
            log::error!("failed to ingest {}: {}", channel.title, e);
        }
    }

    log::debug!("saving subscriptions");
    manager.save()?;

    feed_refresh(&manager, &window).await
}

async fn feed_refresh(manager: &feed::Manager, window: &tauri::Window) -> CommandResult<()> {
    window.emit("feed-refresh", manager.subscriptions())?;
    Ok(())
}
