//! A fast, key-based TODO manager built for developers.
//!
//! This application provides a command-line interface for managing tasks
//! with unique keys for fast, predictable operations.

mod commands; // Command-line interface and argument parsing
mod model; // Data models for tasks and application state
mod storage; // File-based storage using TOML format
mod utils; // Utility functions for date parsing and formatting

use clap::Parser;
use commands::{handle_commands, Args};

fn main() {
    let args = Args::parse();

    let mut storage = storage::Storage::new();
    handle_commands(args, &mut storage);
}
