use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
    str::FromStr,
};

use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

/// This is the in memory manager to handle storing subscriptions and stories.
/// The ingest method takes a channel and converts the items into stories and
/// adds the channel name/link to the subscriptions. Both of these are stored in
/// hash sets so they can be upserted easily.
///
/// Some day this can be replaced with a real database.
#[derive(Debug, Default)]
pub struct Manager {
    directory: PathBuf,
    subscriptions: HashMap<String, Subscription>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ingest error: {0}")]
    Ingest(#[from] TryFromError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl Manager {
    const FILENAME: &str = "subscriptions.json";

    pub fn load(directory: PathBuf) -> Self {
        let file_path = directory.join(Self::FILENAME);

        let subscriptions = if file_path.exists() {
            log::debug!("opening {file_path:?}");

            match File::open(directory.join(Self::FILENAME)) {
                Ok(file) => match serde_json::from_reader(BufReader::new(file)) {
                    Ok(subscriptions) => subscriptions,
                    Err(e) => {
                        log::warn!("failed to parse: {e}");
                        Default::default()
                    }
                },
                Err(e) => {
                    log::warn!("failed to open: {e}");
                    Default::default()
                }
            }
        } else {
            Default::default()
        };

        Self {
            directory,
            subscriptions,
        }
    }

    pub fn ingest(&mut self, url: &str, channel: &rss::Channel) -> Result<(), Error> {
        let name = channel.title();
        log::debug!("ingesting {name}");

        let subscription = self
            .subscriptions
            .entry(name.to_string())
            .or_insert_with(|| Subscription {
                name: name.to_string(),
                url: url.to_string(),

                stories: HashMap::default(),
            });

        for item in channel.items() {
            let story = Story::try_from(item)?;

            subscription
                .stories
                .entry(story.link.clone())
                .or_insert(story);
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), Error> {
        if !self.directory.exists() {
            create_dir_all(&self.directory)?;
        }

        let file = File::create(self.directory.join(Self::FILENAME))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.subscriptions)?;

        Ok(())
    }

    pub fn subscriptions(&self) -> Vec<Subscription> {
        let mut subscriptions: Vec<Subscription> = self.subscriptions.values().cloned().collect();

        subscriptions.sort_by(|a, b| a.name.cmp(&b.name));

        subscriptions
    }

    pub fn stories(&self) -> Vec<Story> {
        let mut stories: Vec<Story> = self
            .subscriptions
            .values()
            .flat_map(|s| s.stories.values())
            .cloned()
            .collect();

        stories.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

        stories
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct Subscription {
    pub name: String,
    pub url: String,

    stories: HashMap<String, Story>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    title: String,
    link: String,
    content: String,
    pub_date: DateTime<Local>,

    starred: bool,
    read: bool,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TryFromError {
    #[error("missing title")]
    MissingTitle,

    #[error("missing link")]
    MissingLink,

    #[error("missing content")]
    MissingContent,

    #[error("missing pub date")]
    MissingPubDate,

    #[error("invalid pub date")]
    ParsePubDate,
}

impl TryFrom<&crate::rss::Item> for Story {
    type Error = TryFromError;
    fn try_from(item: &crate::rss::Item) -> Result<Self, Self::Error> {
        let title = item.title().ok_or(TryFromError::MissingTitle)?;
        let link = item.link().ok_or(TryFromError::MissingLink)?;
        let content = item
            .content()
            .or(item.description())
            .ok_or(TryFromError::MissingContent)?;
        let pub_date = item.pub_date().ok_or(TryFromError::MissingPubDate)?;

        let pub_date = parse_date(pub_date)?;

        let story = Self {
            title: title.to_string(),
            link: link.to_string(),
            content: ammonia::clean(content),
            pub_date,

            starred: false,
            read: false,
        };

        Ok(story)
    }
}

fn parse_date(date: &str) -> Result<DateTime<Local>, TryFromError> {
    let parsers = [
        FromStr::from_str,
        DateTime::parse_from_rfc2822,
        DateTime::parse_from_rfc3339,
        |s: &str| {
            let datetime = NaiveDate::parse_from_str(s, "%Y-%m-%d")?
                .and_time(Default::default())
                .and_local_timezone(Local)
                .unwrap();

            Ok(datetime.into())
        },
    ];

    for parser in parsers {
        if let Ok(date) = parser(date) {
            return Ok(date.into());
        }
    }

    log::error!("failed to parse date: {}", date);
    Err(TryFromError::ParsePubDate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_ingests_channels() -> anyhow::Result<()> {
        let mut item = rss::Item::default();
        item.set_title("Test Item".to_string());
        item.set_link("http://example.com".to_string());
        item.set_description("Test Content".to_string());
        item.set_pub_date("Wed, 15 Mar 2023 04:00:00 -0000".to_string());

        let mut channel = rss::Channel::default();
        channel.set_title("Test Channel");
        channel.set_items(vec![item]);

        let mut manager = Manager::default();
        manager.ingest("http://example.com/feed", &channel)?;

        let subscriptions = manager.subscriptions();
        assert_eq!(subscriptions.len(), 1);
        let subscription = &subscriptions[0];
        assert_eq!(subscription.name, "Test Channel");
        assert_eq!(subscription.url, "http://example.com/feed");

        let stories = manager.stories();
        assert_eq!(stories.len(), 1);
        let story = &stories[0];
        assert_eq!(story.title, "Test Item");
        assert_eq!(story.link, "http://example.com");
        assert_eq!(story.content, "Test Content");
        assert_eq!(
            story.pub_date,
            "2023-03-15T04:00:00+00:00".parse::<DateTime<Local>>()?
        );

        Ok(())
    }

    #[test]
    fn it_converts_item_to_story() -> anyhow::Result<()> {
        let mut item = rss::Item::default();
        assert_eq!(Story::try_from(&item), Err(TryFromError::MissingTitle));
        item.set_title("Test Item".to_string());
        assert_eq!(Story::try_from(&item), Err(TryFromError::MissingLink));
        item.set_link("http://example.com".to_string());
        assert_eq!(Story::try_from(&item), Err(TryFromError::MissingContent));
        item.set_description("Test Description".to_string());
        assert_eq!(Story::try_from(&item), Err(TryFromError::MissingPubDate));
        item.set_pub_date("Wed, 15 Mar 2023 04:00:00 -0000".to_string());

        let story = Story::try_from(&item)?;
        assert_eq!(story.title, "Test Item");
        assert_eq!(story.link, "http://example.com");
        assert_eq!(story.content, "Test Description");
        assert_eq!(
            story.pub_date,
            "2023-03-15T04:00:00+00:00".parse::<DateTime<Local>>()?
        );

        Ok(())
    }

    #[test]
    fn it_parses_date_formats() {
        assert!(parse_date("2023-03-15T04:00:00+00:00").is_ok());
        assert!(parse_date("Wed, 15 Mar 2023 04:00:00 -0000").is_ok());
        assert_eq!(parse_date("not a date"), Err(TryFromError::ParsePubDate));
    }
}
