use feed_rs::parser;
use log::error;

use crate::fetch_url;

pub async fn list_entries(feed_url: &str) {
    let xml = fetch_url(feed_url).await;
    match xml {
        Ok(content) => {
            let feed = parser::parse(content.as_bytes()).unwrap();
            match feed.title {
                Some(title) => println!("Feed Title: {}", title.content),
                None => println!("No title found in the feed."),
            }
            println!("===");
            for (index, entry) in feed.entries.iter().enumerate() {
                println!("#{}: {}", index, entry.title.as_ref().unwrap().content);
                // if let Some(summary) = entry.summary {
                //     println!("Entry Summary: {}", summary.content);
                // }
                println!("---");
            }
        }
        Err(e) => {
            error!("Error fetching feed: {}", e);
            println!("Error fetching feed: {}", e);
        }
    }
}
