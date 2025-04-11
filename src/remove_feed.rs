use log::trace;

pub fn remove_feed(feeds: &mut Vec<String>, feed_id: usize) -> Result<(), &'static str> {
    trace!("Removing feed with ID: {}", feed_id);
    // Check if the feed ID is valid
    if feed_id == 0 || feed_id > feeds.len() {
        return Err("Invalid feed ID");
    }

    // Remove the feed from the list
    let removed_feed = feeds.remove(feed_id - 1);
    trace!("Removed feed: {}", removed_feed);
    println!("Removed feed: {}", removed_feed);

    Ok(())
}
