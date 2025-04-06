use clap::Parser;
use gremlin::{entry_list, feed_list, load_feeds};

#[derive(Clone, Debug, clap::ValueEnum)]
enum Command {
    Ls,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    /// The command to execute
    #[arg(value_enum)]
    command: Command,

    /// The ID of the feed to operate on (required for some commands)
    feed_id: Option<usize>,

    /// The path to the feeds database file
    #[arg(short, long, default_value = "db/feeds.grem")]
    db_path: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let db_path = args.db_path;

    match args.command {
        Command::Ls => {
            // List the feeds
            println!("Listing feeds from: {}", db_path);
            match args.feed_id {
                Some(feed_id) => {
                    // If a specific feed ID is provided, list entries for that feed
                    println!("Listing entries for feed ID: {}", feed_id);
                    let feeds = load_feeds::load_feeds(&db_path);
                    if feed_id == 0 || feed_id >= feeds.len() {
                        println!(
                            "Invalid feed ID: {}. Please provide a valid feed ID between 1 and {}.",
                            feed_id,
                            feeds.len()
                        );
                        return Err(());
                    }
                    entry_list::list(&feeds[feed_id - 1]).await;
                }
                None => {
                    // If no feed ID is provided, list all feeds
                    println!("No specific feed ID provided, listing all feeds.");
                    feed_list::list(&db_path);
                }
            }
        }
    }
    Ok(())
}
