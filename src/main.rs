use gremlin::{entry_list, feed_list, load_feeds};

#[tokio::main]
async fn main() -> Result<(), ()> {
    feed_list::list("db/feeds.grem");

    let feeds = load_feeds::load_feeds("db/feeds.grem");
    entry_list::list(&feeds[0]).await;

    Ok(())
}
