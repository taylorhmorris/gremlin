use log::{info, trace};
use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_feeds(file_path: &str) -> Vec<String> {
    trace!("Loading feeds from file: {}", file_path);
    let mut feeds: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            info!("Read line: {}", line.trim());
            feeds.push(line.trim().to_string());
        }
    }
    feeds
}

pub fn save_feeds(file_path: &str, feeds: Vec<String>) -> Result<(), &'static str> {
    trace!("Saving feeds to file: {}", file_path);
    let mut file = File::create(file_path).map_err(|_| "Failed to create file")?;
    for feed in &feeds {
        writeln!(file, "{}", feed).map_err(|_| "Failed to write to file")?;
        trace!("Wrote feed to file: {}", feed);
    }
    trace!("Saved {} feeds to file", feeds.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_feeds() {
        let temp_file_path = "db/test_feeds.grem";
        std::fs::write(
            temp_file_path,
            "http://example.com/feed1\nhttp://example.com/feed2\nhttp://example.com/feed3\n",
        )
        .unwrap();

        let feeds = load_feeds("db/test_feeds.grem");
        assert_eq!(feeds.len(), 3);
        assert_eq!(feeds[0], "http://example.com/feed1");
        assert_eq!(feeds[1], "http://example.com/feed2");
        assert_eq!(feeds[2], "http://example.com/feed3");

        std::fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_load_feeds_empty_file() {
        // Create a temporary file for testing
        let temp_file_path = "db/empty_test_feeds.grem";
        std::fs::write(temp_file_path, "").unwrap();

        let feeds = load_feeds(temp_file_path);
        assert_eq!(feeds.len(), 0, "Expected no feeds from an empty file.");

        // Clean up the temporary file
        std::fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_load_feeds_non_existent_file() {
        // Test loading from a non-existent file
        let feeds = load_feeds("db/non_existent_file.grem");
        assert_eq!(
            feeds.len(),
            0,
            "Expected no feeds from a non-existent file."
        );
        // No panic or error should occur, just an empty vector returned
    }
}
