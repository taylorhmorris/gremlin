use crate::{list_entries, list_feeds};

pub async fn list(feeds: &[String], feed_id: Option<usize>) {
    if feeds.is_empty() {
        println!("No feeds found.");
        return;
    }

    match feed_id {
        Some(id) if id > 0 && id <= feeds.len() => {
            // List entries for the specified feed ID
            let feed = &feeds[id - 1];
            println!("Feed: {}", feed);
            list_entries::list_entries(feed).await;
        }
        Some(id) => {
            println!(
                "Invalid feed ID: {}. Please provide a valid feed ID between 1 and {}.",
                id,
                feeds.len()
            );
        }
        None => {
            // List all feeds
            list_feeds::list_feeds(feeds);
        }
    }
}
