mod db;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(version = "1.0")]
struct CLI {
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
    let cli = CLI::parse();

    match &cli.command {
        Commands::Add { item } => {
            println!("Adding");
        }

        Commands::List => {
            println!("Listing");
        }

        Commands::Complete { id } => {
            println!("Completed");
        }

        Commands::Delete { id } => {
            println!("Deleting");
        }
    }
}
