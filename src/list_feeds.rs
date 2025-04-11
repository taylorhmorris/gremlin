use std::io::Write;

pub fn list_feeds(feeds: &[String], writer: &mut dyn Write) -> std::io::Result<()> {
    // Load the feeds from the specified database path
    if feeds.is_empty() {
        writeln!(writer, "No feeds found in the database.")?;
    } else {
        writeln!(writer, "List of Feeds:")?;
        for (index, feed) in feeds.iter().enumerate() {
            writeln!(writer, "{}: {}", index + 1, feed)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_feeds() -> std::io::Result<()> {
        let feeds = vec![
            "https://example.com/feed1".to_string(),
            "https://example.com/feed2".to_string(),
        ];
        let mut handle = Vec::new();
        list_feeds(&feeds, &mut handle)?;
        let output = String::from_utf8(handle).unwrap();
        assert!(output.contains("List of Feeds:"));
        assert!(output.contains("1: https://example.com/feed1"));
        assert!(output.contains("2: https://example.com/feed2"));
        Ok(())
    }

    #[test]
    fn test_list_feeds_empty() -> std::io::Result<()> {
        let feeds: Vec<String> = Vec::new();
        let mut handle = Vec::new();
        list_feeds(&feeds, &mut handle)?;
        let output = String::from_utf8(handle).unwrap();
        assert!(output.contains("No feeds found in the database."));
        Ok(())
    }
}
