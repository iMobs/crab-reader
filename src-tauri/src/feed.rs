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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Subscription {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Story {
    title: String,
    link: String,
    description: String,
    pub_date: DateTime<Local>,
}

#[derive(Debug, thiserror::Error)]
pub enum TryFromError {
    #[error("missing title")]
    MissingTitle,

    #[error("missing link")]
    MissingLink,

    #[error("missing description")]
    MissingDescription,

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
        let description = item.description().ok_or(TryFromError::MissingDescription)?;
        let pub_date = item.pub_date().ok_or(TryFromError::MissingPubDate)?;

        let pub_date = parse_date(pub_date)?;

        let story = Self {
            title: title.to_string(),
            link: link.to_string(),
            description: description.to_string(),
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
