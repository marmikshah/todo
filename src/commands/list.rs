use crate::{model::task::TaskStatus, storage::Storage};
use colored::*;
use comfy_table::{presets::UTF8_FULL, Cell, Table};

/// Displays tasks in a formatted table with optional filtering.
///
/// Tasks are displayed with their keys, descriptions, status, priority, and due dates.
/// When verbose mode is enabled, creation and update timestamps are also shown.
///
/// # Arguments
///
/// * `storage` - Reference to the storage system
/// * `filter` - Optional text filter (searches within task descriptions)
/// * `status` - Optional status filter
/// * `verbose` - Whether to show additional timestamp columns
/// * `_limit` - Optional limit on number of results (currently unused)
pub fn handle_list(
    storage: &Storage,
    filter: Option<String>,
    status: Option<TaskStatus>,
    verbose: bool,
    _limit: Option<usize>,
) {
    let tasks: Vec<_> = storage
        .data
        .tasks
        .iter()
        .filter(|(_, task)| {
            let mut matches = true;

            if let Some(status) = &status {
                matches &= task.status == *status;
            }

            if let Some(filter) = &filter {
                matches &= task.description.contains(filter);
            }

            matches
        })
        .collect();

    if tasks.is_empty() {
        println!("{}", "No tasks found.".yellow());
        return;
    }

    let task_count = tasks.len();
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    let mut headers = vec![
        Cell::new("Key")
            .fg(comfy_table::Color::Cyan)
            .add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Description")
            .fg(comfy_table::Color::White)
            .add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Status")
            .fg(comfy_table::Color::Green)
            .add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Priority")
            .fg(comfy_table::Color::Yellow)
            .add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Due Date")
            .fg(comfy_table::Color::Blue)
            .add_attribute(comfy_table::Attribute::Bold),
    ];
    if verbose {
        headers.push(
            Cell::new("Created At")
                .fg(comfy_table::Color::Blue)
                .add_attribute(comfy_table::Attribute::Bold),
        );
        headers.push(
            Cell::new("Updated At")
                .fg(comfy_table::Color::Blue)
                .add_attribute(comfy_table::Attribute::Bold),
        );
    }

    table.set_header(headers);

    for (key, task) in tasks {
        let mut row = vec![
            Cell::new(key.as_str()),
            Cell::new(task.description.as_str()),
            task.status.get_comfy_cell(),
            task.priority.get_comfy_cell(),
            Cell::new(task.due_date.format("%Y%m%d").to_string()),
        ];

        if verbose {
            row.push(Cell::new(task.created_at.format("%Y%m%d").to_string()));
            row.push(Cell::new(task.updated_at.format("%Y%m%d").to_string()));
        }

        table.add_row(row);
    }

    println!("{}", table);
    println!("{}", format!("{} tasks found.", task_count).dimmed());
}
