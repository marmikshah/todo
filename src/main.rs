mod db;

use clap::{Parser, Subcommand};

use log::debug;
use todo::{commands::init::initialise_tables, config::Config};
use env_logger::Env;
use std::{env};


#[derive(Parser)]
#[command(name = "todo")]
#[command(version = "0.0.0")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { item: String },
    List,
    Complete { id: i32 },
    Delete { id: i32 },
}

fn main() {
    let args = Args::parse();

    let mut config = Config::default();

    let loglevel = env::var("TODO_LOG_LEVEL").unwrap_or_else(|_| String::from("debug"));
    env_logger::Builder::from_env(Env::default().default_filter_or(&loglevel)).init();


    match &args.command {
        Commands::Init => {
            initialise_tables(&mut config);
        }

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
