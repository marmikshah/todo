mod db;

use clap::{Parser, Subcommand};

use log::debug;
use todo::APP_CONFIG;

#[derive(Parser)]
#[command(name = "todo")]
#[command(version = "0.0.0")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { item: String },
    List,
    Complete { id: i32 },
    Delete { id: i32 },
}

fn main() {
    let args = Args::parse();

    let config = APP_CONFIG.lock().unwrap();

    match &args.command {
        Commands::Add { item } => {
            println!("Adding {}", item);
        }

        Commands::List => {
            println!("Listing");
            let path = &config.path;
            debug!("Path: {path}");
        }

        Commands::Complete { id } => {
            println!("Completed: {}", id);
        }

        Commands::Delete { id } => {
            println!("Deleting: {}", id);
        }
    }
}
