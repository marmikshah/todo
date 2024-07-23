mod commands;
mod config;
mod db;

use clap::{Parser, Subcommand};

use commands::complete::complete_task;
use commands::deinit::deinit;
use commands::delete::delete_task;
use commands::{add::add_task, list::list_tasks};
use config::ConfigError;
use env_logger::Env;
use log::{debug, error};
use std::env;
use std::io::Write;
use std::process::exit;

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
    Deinit,
}

fn exit_if_needed(setup_status: Result<(), ConfigError>) {
    debug!("Setup Status: {}", setup_status.is_ok());
    if setup_status.is_err() {
        error!("Please run `todo init` first.");
        exit(1);
    }
}

fn main() {
    let args = Args::parse();

    let config = config::Config::default();

    let status = config.get_setup_status(true);

    let loglevel = env::var("TODO_LOG_LEVEL").unwrap_or_else(|_| String::from("info"));
    env_logger::Builder::from_env(Env::default().default_filter_or(&loglevel))
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    match &args.command {
        Commands::Init => {
            // For `init` db path will not exist in the beginning,
            // so no need to check for setup status.
            let _ = commands::init::init();
        }

        Commands::Add { item } => {
            exit_if_needed(status);
            add_task(item);
            list_tasks();
        }

        Commands::List => {
            exit_if_needed(status);
            list_tasks();
        }

        Commands::Complete { id } => {
            exit_if_needed(status);
            if complete_task(id).is_ok() {
                list_tasks();
            }
        }

        Commands::Delete { id } => {
            exit_if_needed(status);
            if delete_task(id).is_ok() {
                list_tasks();
            }
        }

        Commands::Deinit => {
            exit_if_needed(status);
            if deinit().is_err() {
                list_tasks();
            }
        }
    }
}
