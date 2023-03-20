use std::{collections::HashSet, str::FromStr};

use chrono::{DateTime, Local};
use serde::Serialize;

/// This is the in memory manager to handle storing subscriptions and stories.
/// The ingest method takes a channel and converts the items into stories and
/// adds the channel name/link to the subscriptions. Both of these are stored in
/// hash sets so they can be upserted easily.
///
/// Some day this can be replaced with a real database.
#[derive(Debug, Default)]
pub struct Manager {
    subscriptions: HashSet<Subscription>,
    stories: HashSet<Story>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ingest error: {0}")]
    Ingest(#[from] TryFromError),
}

impl Manager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ingest(&mut self, channel: &rss::Channel) -> Result<(), Error> {
        let stories = channel
            .items
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<HashSet<Story>, TryFromError>>()
            .map_err(Error::Ingest)?;

        self.stories.extend(stories);
        self.subscriptions.insert(Subscription {
            name: channel.title.clone(),
            url: channel.link.clone(),
        });

        Ok(())
    }

    pub fn subscriptions(&self) -> Vec<Subscription> {
        let mut subscriptions: Vec<Subscription> = self.subscriptions.iter().cloned().collect();

        subscriptions.sort_by(|a, b| a.name.cmp(&b.name));

        subscriptions
    }

    pub fn stories(&self) -> Vec<Story> {
        let mut stories: Vec<Story> = self.stories.iter().cloned().collect();

        stories.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

        stories
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, specta::Type)]
pub struct Subscription {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, specta::Type)]
pub struct Story {
    title: String,
    link: String,
    content: String,
    pub_date: DateTime<Local>,
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
            content: content.to_string(),
            pub_date,
        };

        Ok(story)
    }
}

fn parse_date(date: &str) -> Result<DateTime<Local>, TryFromError> {
    let parsers = [
        FromStr::from_str,
        DateTime::parse_from_rfc2822,
        DateTime::parse_from_rfc3339,
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
        channel.set_link("http://example.com/feed");
        channel.set_items(vec![item]);

        let mut manager = Manager::new();
        manager.ingest(&channel)?;

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
