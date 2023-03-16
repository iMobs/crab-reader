use std::collections::HashSet;

use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Default)]
pub struct Manager {
    subscriptions: HashSet<Subscription>,
    stories: HashSet<Story>,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] TryFromError);

impl Manager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ingest(&mut self, channel: rss::Channel) -> Result<(), Error> {
        let stories = channel
            .items
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<HashSet<Story>, TryFromError>>()
            .map_err(Error)?;

        self.stories.extend(stories);
        self.subscriptions.insert(Subscription {
            name: channel.title,
            url: channel.link,
        });

        Ok(())
    }

    pub fn subscriptions(&self) -> Vec<Subscription> {
        let mut subscriptions: Vec<Subscription> = self.subscriptions.clone().into_iter().collect();

        subscriptions.sort_by(|a, b| a.name.cmp(&b.name));

        subscriptions
    }

    pub fn stories(&self) -> Vec<Story> {
        let mut stories: Vec<Story> = self.stories.clone().into_iter().collect();

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

impl TryFrom<crate::rss::Item> for Story {
    type Error = TryFromError;
    fn try_from(item: crate::rss::Item) -> Result<Self, Self::Error> {
        let Some(title) = item.title else {
            return Err(TryFromError::MissingTitle)
        };

        let Some(link) = item.link else {
            return Err(TryFromError::MissingLink);
        };

        let Some(description) = item.description else {
            return Err(TryFromError::MissingDescription)
        };

        let Some(pub_date) = item.pub_date else {
            return Err(TryFromError::MissingPubDate);
        };

        let pub_date = match pub_date.parse() {
            Ok(d) => d,
            Err(_) => match DateTime::parse_from_rfc2822(&pub_date) {
                Ok(d) => d,
                Err(_) => match DateTime::parse_from_rfc3339(&pub_date) {
                    Ok(d) => d,
                    Err(_) => return Err(TryFromError::ParsePubDate),
                },
            },
        };

        let story = Self {
            title,
            link,
            description,
            pub_date: pub_date.into(),
        };

        Ok(story)
    }
}
