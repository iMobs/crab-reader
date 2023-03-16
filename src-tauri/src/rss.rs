pub use rss::{Channel, Item};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Rss(#[from] rss::Error),
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn get_channels(urls: &[String]) -> Result<Vec<Channel>> {
    futures::future::try_join_all(urls.iter().map(|url| get_channel(url))).await
}

pub async fn get_channel(url: &str) -> Result<Channel> {
    let raw = reqwest::get(url).await?.bytes().await?;
    let channel = rss::Channel::read_from(raw.as_ref())?;

    Ok(channel)
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

        let result = get_channels(&[url]).await?;
        mock.assert_async().await;

        assert_eq!(result.len(), 1);

        let channel = &result[0];
        assert_eq!(channel.title(), "xkcd.com");
        assert_eq!(channel.link(), "https://xkcd.com/");
        assert_eq!(
            channel.description(),
            "xkcd.com: A webcomic of romance and math humor."
        );
        assert_eq!(channel.language(), Some("en"));

        let items = channel.items();
        assert_eq!(items.len(), 4);

        let item = &items[0];
        assert_eq!(item.title(), Some("Flatten the Planets"));
        assert_eq!(item.link(), Some("https://xkcd.com/2750/"));
        assert_eq!(
            item.description(),
            Some(
                r#"<img src="https://imgs.xkcd.com/comics/flatten_the_planets.png" title="We'll turn the asteroid belt into ball bearings to go between different rings orbiting at different speeds." alt="We'll turn the asteroid belt into ball bearings to go between different rings orbiting at different speeds." />"#
            )
        );

        Ok(())
    }
}
