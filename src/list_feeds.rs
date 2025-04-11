pub fn list_feeds(feeds: &[String]) {
    // Load the feeds from the specified database path
    if feeds.is_empty() {
        println!("No feeds found in the database.");
    } else {
        println!("List of Feeds:");
        for (index, feed) in feeds.iter().enumerate() {
            println!("{}: {}", index + 1, feed);
        }
    }
}
