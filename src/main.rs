mod commands;
mod config;
mod db;

use clap::{Parser, Subcommand};

use commands::{add::add_task, list::list_tasks};
use env_logger::Env;
use std::env;
use std::io::Write;

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

    let config = config::Config::new();

    let loglevel = env::var("TODO_LOG_LEVEL").unwrap_or_else(|_| String::from("info"));
    env_logger::Builder::from_env(Env::default().default_filter_or(&loglevel))
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    match &args.command {
        Commands::Init => {
            let _ = commands::init::init();
        }

        Commands::Add { item } => {
            println!("Adding {}", item);
            add_task(item)
        }

        Commands::List => {
            println!("Listing");
            list_tasks();
        }

        Commands::Complete { id } => {
            println!("Completed: {}", id);
        }

        Commands::Delete { id } => {
            println!("Deleting: {}", id);
        }
    }
}
