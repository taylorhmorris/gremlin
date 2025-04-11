use clap::Parser;
use gremlin::{
    add_feed, list,
    load_feeds::{self, save_feeds},
};
use log::{error, trace};
use std::env;

#[derive(Clone, Debug, clap::Subcommand)]
enum Command {
    Add { url: String },
    Ls,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    /// The command to execute
    #[command[subcommand]]
    command: Command,

    /// The ID of the feed to operate on (required for some commands)
    feed_id: Option<usize>,

    /// The path to the feeds database file
    #[arg(short, long, default_value = "db/feeds.grem")]
    db_path: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let log_level = match env::var("RUST_LOG").unwrap_or("info".to_string()).as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "off" => log::LevelFilter::Off,
        _ => log::LevelFilter::Warn,
    };
    let mut clog = colog::default_builder();
    clog.filter(None, log_level);
    clog.init();
    trace!("Log level set to: {}", log_level);
    trace!("Starting gremlin...");
    let args = Args::parse();
    let db_path = args.db_path;

    let mut feeds = load_feeds::load_feeds(&db_path);
    match args.command {
        Command::Add { url } => {
            trace!(target: "main", "Adding URL: {}", url);
            let result = add_feed::add_feed(&mut feeds, &url).await;
            match result {
                Ok(_) => println!("Feed added successfully."),
                Err(e) => error!("Error adding feed: {}", e),
            }
        }
        Command::Ls => {
            list::list(&feeds, args.feed_id).await;
        }
    }

    let result = save_feeds(&db_path, feeds);
    match result {
        Ok(_) => println!("Feeds saved successfully."),
        Err(e) => {
            error!("Error saving feeds: {}", e);
            eprintln!("Could not save feeds: {}", e);
        }
    }

    Ok(())
}
