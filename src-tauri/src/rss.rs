use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Xml(#[from] serde_xml_rs::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn get_feeds(urls: &[String]) -> Result<Vec<RssFeed>> {
    futures::future::try_join_all(urls.iter().map(|url| get_feed(url))).await
}

pub async fn get_feed(url: &str) -> Result<RssFeed> {
    let raw = reqwest::get(url).await?.text().await?;
    let feed: RssFeed = from_str(&raw)?;
    Ok(feed)
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RssFeed {
    channel: RssChannel,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct RssChannel {
    title: String,
    link: String,
    description: String,
    language: String,
    item: Vec<RssItem>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RssItem {
    guid: String,
    title: String,
    author: Option<String>,
    link: String,
    #[serde(deserialize_with = "rfc2822_date_format::deserialize")]
    pub_date: DateTime<Local>,
    // description: String,
}

mod rfc2822_date_format {
    use chrono::{DateTime, Local};
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        DateTime::parse_from_rfc2822(&s)
            .map(Into::into)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_feeds() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = format!("{}/feed.xml", server.url());

        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
        <rss version="2.0">
            <channel>
                <title>xkcd.com</title>
                <link>https://xkcd.com/</link>
                <description>xkcd.com: A webcomic of romance and math humor.</description>
                <language>en</language>
                <item>
                    <title>Lymphocytes</title>
                    <link>https://xkcd.com/2749/</link>
                    <description>
                        <img src="https://imgs.xkcd.com/comics/lymphocytes.png" title="It's very hard to detect, but recent studies have determined that when plasma B cells are producing antibodies, they go 'pew pew pew'" alt="It's very hard to detect, but recent studies have determined that when plasma B cells are producing antibodies, they go 'pew pew pew'" />
                    </description>
                    <pubDate>Mon, 13 Mar 2023 04:00:00 -0000</pubDate>
                    <guid>https://xkcd.com/2749/</guid>
                </item>
            </channel>
        </rss>"#.to_string();

        let mock = server
            .mock("GET", "/feed.xml")
            .with_status(200)
            .with_header("content-type", "text/xml")
            .with_body(&xml)
            .create_async()
            .await;

        let response = get_feeds(&[url]).await?;

        assert_eq!(
            response,
            [RssFeed {
                channel: RssChannel {
                    title: "xkcd.com".into(),
                    link: "https://xkcd.com/".into(),
                    description: "xkcd.com: A webcomic of romance and math humor.".into(),
                    language: "en".into(),
                    item: vec![RssItem {
                        title: "Lymphocytes".into(),
                        author: None,
                        link: "https://xkcd.com/2749/".into(),
                        pub_date: DateTime::parse_from_rfc2822("Mon, 13 Mar 2023 04:00:00 -0000")?
                            .into(),
                        guid: "https://xkcd.com/2749/".into(),
                        // description: r#"<img src="https://imgs.xkcd.com/comics/lymphocytes.png" title="It's very hard to detect, but recent studies have determined that when plasma B cells are producing antibodies, they go 'pew pew pew'" alt="It's very hard to detect, but recent studies have determined that when plasma B cells are producing antibodies, they go 'pew pew pew'" />"#.into(),
                    }]
                }
            }]
        );

        mock.assert_async().await;

        Ok(())
    }
}
