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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_feed() {
        let mut feeds = vec![
            "https://example.com/feed1".to_string(),
            "https://example.com/feed2".to_string(),
            "https://example.com/feed3".to_string(),
        ];

        // Test removing a valid feed
        assert_eq!(remove_feed(&mut feeds, 2), Ok(()));
        assert_eq!(
            feeds,
            vec![
                "https://example.com/feed1".to_string(),
                "https://example.com/feed3".to_string(),
            ]
        );

        // Test removing an invalid feed ID
        assert_eq!(remove_feed(&mut feeds, 5), Err("Invalid feed ID"));
    }
}
