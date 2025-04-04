use crate::load_feeds::load_feeds;

pub fn list(db_path: &str) -> () {
    // Load the feeds from the specified database path
    let feeds = load_feeds(db_path);

    if feeds.is_empty() {
        println!("No feeds found in the database.");
    } else {
        println!("List of Feeds:");
        for (index, feed) in feeds.iter().enumerate() {
            println!("{}: {}", index + 1, feed);
        }
    }
}
