use chrono::{DateTime, Local};
use quick_xml::de::{from_str, DeError};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Xml(#[from] DeError),
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
    let feed = from_str(&raw)?;

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
    description: String,
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

    // XKCD is always a good example
    const XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <rss version="2.0"><channel><title>xkcd.com</title><link>https://xkcd.com/</link><description>xkcd.com: A webcomic of romance and math humor.</description><language>en</language><item><title>Flatten the Planets</title><link>https://xkcd.com/2750/</link><description>&lt;img src="https://imgs.xkcd.com/comics/flatten_the_planets.png" title="We'll turn the asteroid belt into ball bearings to go between different rings orbiting at different speeds." alt="We'll turn the asteroid belt into ball bearings to go between different rings orbiting at different speeds." /&gt;</description><pubDate>Wed, 15 Mar 2023 04:00:00 -0000</pubDate><guid>https://xkcd.com/2750/</guid></item><item><title>Lymphocytes</title><link>https://xkcd.com/2749/</link><description>&lt;img src="https://imgs.xkcd.com/comics/lymphocytes.png" title="It's very hard to detect, but recent studies have determined that when plasma B cells are producing antibodies, they go 'pew pew pew'" alt="It's very hard to detect, but recent studies have determined that when plasma B cells are producing antibodies, they go 'pew pew pew'" /&gt;</description><pubDate>Mon, 13 Mar 2023 04:00:00 -0000</pubDate><guid>https://xkcd.com/2749/</guid></item><item><title>Radians are Cursed</title><link>https://xkcd.com/2748/</link><description>&lt;img src="https://imgs.xkcd.com/comics/radians_are_cursed.png" title="Phil Plait once pointed out that you can calculate the total angular area of the sky this way. If the sky is a sphere with radius 57.3 degrees, then its area is 4*pi*r^2=41,253 square degrees. This makes dimensional analysts SO mad, but you can't argue with results." alt="Phil Plait once pointed out that you can calculate the total angular area of the sky this way. If the sky is a sphere with radius 57.3 degrees, then its area is 4*pi*r^2=41,253 square degrees. This makes dimensional analysts SO mad, but you can't argue with results." /&gt;</description><pubDate>Fri, 10 Mar 2023 05:00:00 -0000</pubDate><guid>https://xkcd.com/2748/</guid></item><item><title>Presents for Biologists</title><link>https://xkcd.com/2747/</link><description>&lt;img src="https://imgs.xkcd.com/comics/presents_for_biologists.png" title="A lot of these are actually non-venomous, but I can see which species you mistook them for. If you pause the crane for a sec I can give you some ID pointers for next time!" alt="A lot of these are actually non-venomous, but I can see which species you mistook them for. If you pause the crane for a sec I can give you some ID pointers for next time!" /&gt;</description><pubDate>Wed, 08 Mar 2023 05:00:00 -0000</pubDate><guid>https://xkcd.com/2747/</guid></item></channel></rss>"#;

    #[tokio::test]
    async fn fetch_feeds() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = format!("{}/feed.xml", server.url());

        let mock = server
            .mock("GET", "/feed.xml")
            .with_status(200)
            .with_header("content-type", "text/xml")
            .with_body(XML)
            .create_async()
            .await;

        let response = get_feeds(&[url]).await?;
        mock.assert_async().await;

        assert_eq!(response.len(), 1);
        let feed = &response[0];
        assert_eq!(feed.channel.title, "xkcd.com");
        assert_eq!(feed.channel.link, "https://xkcd.com/");
        assert_eq!(
            feed.channel.description,
            "xkcd.com: A webcomic of romance and math humor."
        );
        assert_eq!(feed.channel.language, "en");
        assert_eq!(feed.channel.item.len(), 4);
        let item = &feed.channel.item[0];
        assert_eq!(item.title, "Flatten the Planets");
        assert_eq!(item.author, None);
        assert_eq!(item.link, "https://xkcd.com/2750/");
        assert_eq!(
            item.pub_date,
            DateTime::parse_from_rfc2822("Wed, 15 Mar 2023 04:00:00 -0000")?,
        );
        assert_eq!(item.guid, "https://xkcd.com/2750/");
        assert_eq!(
            item.description,
            r#"<img src="https://imgs.xkcd.com/comics/flatten_the_planets.png" title="We'll turn the asteroid belt into ball bearings to go between different rings orbiting at different speeds." alt="We'll turn the asteroid belt into ball bearings to go between different rings orbiting at different speeds." />"#
        );

        Ok(())
    }
}
