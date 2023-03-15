pub async fn get_feeds(urls: &[String]) -> anyhow::Result<Vec<String>> {
    futures::future::try_join_all(urls.iter().map(|url| get_feed(url))).await
}

pub async fn get_feed(url: &str) -> anyhow::Result<String> {
    let feed = reqwest::get(url).await?.text().await?;
    Ok(feed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_feeds() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = format!("{}/feed.xml", server.url());

        let result = "TEST RESULT".to_string();

        let mock = server
            .mock("GET", "/feed.xml")
            .with_status(200)
            .with_header("content-type", "text/xml")
            .with_body(&result)
            .create_async()
            .await;

        let response = get_feeds(&[url]).await?;

        assert_eq!(response, [result]);

        mock.assert_async().await;

        Ok(())
    }
}
