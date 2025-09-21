//! Data models for tasks and application state.

use chrono::prelude::*;
use clap::ValueEnum;
use comfy_table::Cell;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::get_end_of_today;

/// Represents the current status of a task.
#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum, Default, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task has been created but work has not yet begun
    #[default]
    NotStarted,
    /// Task is currently being worked on
    InProgress,
    /// Task has been temporarily paused
    Paused,
    /// Task has been completed successfully
    Completed,
    /// Task has been cancelled and will not be completed
    Cancelled,
}

impl TaskStatus {
    /// Returns a human-readable string representation of the status.
    pub fn get_str(&self) -> &str {
        match &self {
            TaskStatus::NotStarted => "Not Started",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Paused => "Paused",
            TaskStatus::Completed => "Completed",
            TaskStatus::Cancelled => "Cancelled",
        }
    }

    /// Returns a formatted cell for display in tables with appropriate colors.
    pub fn get_comfy_cell(&self) -> Cell {
        let cell = Cell::new(self.get_str());
        match &self {
            TaskStatus::NotStarted => cell.fg(comfy_table::Color::White),
            TaskStatus::InProgress => cell.fg(comfy_table::Color::Blue),
            TaskStatus::Paused => cell.fg(comfy_table::Color::Yellow),
            TaskStatus::Completed => cell.fg(comfy_table::Color::Green),
            TaskStatus::Cancelled => cell.fg(comfy_table::Color::Red),
        }
    }
}

/// Represents the priority level of a task.
#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum, Default, PartialEq, Eq)]
pub enum TaskPriority {
    /// Low priority task
    Low,
    /// Medium priority task (default)
    #[default]
    Medium,
    /// High priority task
    High,
}

impl TaskPriority {
    /// Returns a human-readable string representation of the priority.
    pub fn get_str(&self) -> &str {
        match &self {
            TaskPriority::Low => "Low",
            TaskPriority::Medium => "Medium",
            TaskPriority::High => "High",
        }
    }

    /// Returns a formatted cell for display in tables with appropriate colors.
    pub fn get_comfy_cell(&self) -> Cell {
        let cell = Cell::new(self.get_str());
        match &self {
            TaskPriority::Low => cell.fg(comfy_table::Color::Green),
            TaskPriority::Medium => cell.fg(comfy_table::Color::Yellow),
            TaskPriority::High => cell.fg(comfy_table::Color::Red),
        }
    }
}

/// A single task item in the TODO application.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskItem {
    /// Unique identifier for this task
    pub id: Uuid,
    /// Human-readable description of the task
    pub description: String,
    /// Current status of the task
    pub status: TaskStatus,
    /// Priority level of the task
    pub priority: TaskPriority,
    /// When this task was created
    pub created_at: DateTime<Local>,
    /// When this task was last updated
    pub updated_at: DateTime<Local>,
    /// When this task is due (defaults to end of today)
    pub due_date: DateTime<Local>,
}

impl Default for TaskItem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            description: String::new(),
            status: TaskStatus::NotStarted,
            priority: TaskPriority::Medium,
            created_at: Local::now(),
            updated_at: Local::now(),
            due_date: get_end_of_today(),
        }
    }
}
