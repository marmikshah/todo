//! Command-line interface and argument parsing.

use crate::{
    model::task::{TaskPriority, TaskStatus},
    storage::Storage,
};
use clap::{Parser, Subcommand};

// Command modules
pub mod add;
pub mod complete;
pub mod delete;
pub mod edit;
pub mod list;

/// A fast, key-based TODO manager built for developers.
///
/// Every task gets a unique key for fast, predictable operations.
/// Keys are user-defined and must be unique. Use `list` with filters
/// to find tasks instead of separate search commands.
///
/// # Usage Examples
///
/// ```bash
/// todo add auth-bug "Fix authentication bug"
/// todo add test-001 "Add tests" --priority high
/// todo list --filter "auth" --status not-started
/// todo complete auth-bug
/// todo delete test-001
/// ```
#[derive(Parser)]
#[command(
    name = "todo",
    version = "0.1.0",
    about = "A fast, key-based TODO manager for developers",
    long_about = r#"
A fast, key-based TODO manager built for developers.

Every task gets a unique key for fast, predictable operations.
Keys are user-defined and must be unique. Use 'list' with filters
to find tasks instead of separate search commands.

Examples:
  todo add auth-bug "Fix authentication bug"
  todo add test-001 "Add tests" --priority high
  todo list --filter "auth" --status not-started
  todo complete auth-bug
  todo delete test-001
    "#
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

/// Main command enum for the TODO CLI.
///
/// All commands are key-based, operating on tasks using their unique keys.
#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task with a unique key.
    ///
    /// # Examples
    ///
    /// ```bash
    /// todo add auth-bug "Fix authentication bug"
    /// todo add test-001 "Add tests" --priority high
    /// ```
    Add {
        /// Key assigned to this task, must be unique
        key: String,

        /// Task description
        description: String,

        /// Priority, defaults to `medium`
        #[arg(short, long, value_enum)]
        priority: Option<TaskPriority>,

        /// Status, defaults to `not-started`
        #[arg(short, long, value_enum)]
        status: Option<TaskStatus>,

        /// Due date in YYYYMMDD format
        #[arg(long)]
        due: Option<String>,
    },

    /// List tasks with optional filtering.
    ///
    /// Supports filtering by description text and status.
    /// This replaces the need for separate search and pattern matching commands.
    ///
    /// # Examples
    ///
    /// ```bash
    /// todo list                                    # Show all tasks
    /// todo list --status completed                 # Show completed tasks
    /// todo list --filter "auth"                    # Show tasks containing "auth"
    /// todo list --filter "bug" --status not-started # Combined filtering
    /// ```
    List {
        /// Filter by description text (searches within task descriptions)
        #[arg(short, long)]
        filter: Option<String>,

        /// Filter by status
        #[arg(short, long, value_enum)]
        status: Option<TaskStatus>,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,

        /// Show more information, (created_at, updated_at)
        #[arg(short, long)]
        verbose: bool,
    },

    /// Mark task as completed.
    Complete {
        /// Task key
        key: String,
    },

    /// Delete a task.
    Delete {
        /// Task key
        key: String,
    },

    /// Edit an existing task.
    ///
    /// # Examples
    ///
    /// ```bash
    /// todo edit auth-bug --description "Fixed properly"
    /// todo edit auth-bug --priority high
    /// ```
    Edit {
        /// Task key
        key: String,

        /// New description
        #[arg(short, long)]
        description: Option<String>,

        /// New due date (YYYYMMDD format)
        #[arg(long)]
        due: Option<String>,

        /// Priority
        #[arg(short, long, value_enum)]
        priority: Option<TaskPriority>,

        /// Status
        #[arg(short, long, value_enum)]
        status: Option<TaskStatus>,
    },
}

/// Dispatches commands to their respective handlers based on the parsed arguments.
///
/// # Arguments
///
/// * `args` - Parsed command-line arguments
/// * `storage` - Mutable reference to the storage system
pub fn handle_commands(args: Args, storage: &mut Storage) {
    match args.command {
        Commands::Add {
            description,
            key,
            priority,
            status,
            due,
        } => {
            add::handle_add(storage, key, description, priority, status, due);
        }
        Commands::List {
            filter,
            status,
            limit,
            verbose,
        } => {
            list::handle_list(storage, filter, status, verbose, limit);
        }
        Commands::Complete { key } => {
            complete::handle_complete(storage, key);
        }
        Commands::Delete { key } => {
            delete::handle_delete(storage, key);
        }
        Commands::Edit {
            key,
            description,
            due,
            priority,
            status,
        } => {
            edit::handle_edit(storage, key, description, priority, status, due);
        }
    }
}
