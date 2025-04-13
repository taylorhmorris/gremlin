use clap::Parser;
use gremlin::{
    add_feed, list,
    load_feeds::{self, save_feeds},
    remove_feed,
};
use log::{error, trace};
use std::env;

#[derive(Clone, Debug, clap::Subcommand)]
enum Command {
    Add { url: String },
    Rm { feed_id: usize },
    Ls { feed_id: Option<usize> },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    /// The command to execute
    #[command[subcommand]]
    command: Command,

    /// The path to the feeds database file
    #[arg(short, long, default_value = "db/")]
    db_path: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let log_level = match env::var("GREMLIN_LOG")
        .unwrap_or("info".to_string())
        .as_str()
    {
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
    let db_path = args.db_path + "feeds.grem";

    let mut dirty: bool = false; // track if we need to save feeds
    let mut feeds = load_feeds::load_feeds(&db_path);
    match args.command {
        Command::Add { url } => {
            trace!(target: "main", "Adding URL: {}", url);
            let result = add_feed::add_feed(&mut feeds, &url).await;
            match result {
                Ok(_) => {
                    dirty = true;
                    println!("Feed added successfully.")
                }
                Err(e) => error!("Error adding feed: {}", e),
            }
        }
        Command::Ls { feed_id } => {
            list::list(&feeds, feed_id).await;
        }
        Command::Rm { feed_id } => {
            trace!(target: "main", "Removing feed with ID: {}", feed_id);
            let result = remove_feed::remove_feed(&mut feeds, feed_id);
            match result {
                Ok(_) => {
                    dirty = true;
                    println!("Feed removed successfully.")
                }
                Err(e) => error!("Error removing feed: {}", e),
            }
        }
    }

    if dirty {
        trace!(target: "main", "Saving feeds to file: {}", db_path);
        let result = save_feeds(&db_path, feeds);
        match result {
            Ok(_) => println!("Feeds saved successfully."),
            Err(e) => {
                error!("Error saving feeds: {}", e);
                eprintln!("Could not save feeds: {}", e);
            }
        }
    }

    Ok(())
}
