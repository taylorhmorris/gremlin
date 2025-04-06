pub mod list;
pub mod list_entries;
pub mod list_feeds;
pub mod load_feeds;

pub async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    // Make an asynchronous GET request to the specified URL
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}
