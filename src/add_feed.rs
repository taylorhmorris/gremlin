use log::trace;

pub async fn add_feed(feeds: &mut Vec<String>, feed_url: &str) -> Result<(), &'static str> {
    trace!("Adding new feed: {}", feed_url);
    // Check if the feed URL already exists
    if feeds.iter().any(|f| f == feed_url) {
        trace!("Feed URL ({}) already exists", feed_url);
        println!("Feed URL already exists: {}", feed_url);
        return Err("Feed URL already exists");
    }

    // Add the new feed URL to the list
    feeds.push(feed_url.to_string());

    trace!("Added new feed: {}", feed_url);
    Ok(())
}
