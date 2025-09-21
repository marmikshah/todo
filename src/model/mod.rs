//! Data models for the TODO application.

pub mod task;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root data structure containing all application data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppData {
    /// Map of task keys to task items
    pub tasks: HashMap<String, task::TaskItem>,
}
