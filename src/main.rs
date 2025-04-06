use clap::Parser;
use gremlin::{list, load_feeds};

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
            let feeds = load_feeds::load_feeds(&db_path);
            list::list(feeds, args.feed_id).await;
        }
    }
    Ok(())
}
