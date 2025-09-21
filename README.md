# TODO CLI

> A fast, key-based TODO manager built for developers

A lightning-fast command-line task manager designed specifically for developers. Every task gets a unique key for predictable, efficient operations.

## ✨ Features

- **🚀 Key-based operations** - Fast, predictable task management with unique keys
- **📊 Beautiful tabular display** - Color-coded status and priority indicators
- **🔍 Smart filtering** - Filter tasks by status, description, or custom criteria
- **📅 Due date management** - Set and track task deadlines with YYYYMMDD format
- **⚡ Priority levels** - Low, Medium, High priority classification
- **💾 TOML storage** - Human-readable, version-control friendly data format

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/marmikshah/todo.git
cd todo
cargo build --release
```

### Basic Usage

```bash
# Add a new task
todo add auth-bug "Fix authentication bug"

# Add a high-priority task with due date
todo add test-001 "Add unit tests" --priority high --due 20241225

# List all tasks
todo list

# Filter tasks by status
todo list --status not-started

# Search tasks by description
todo list --filter "auth"

# Edit a task
todo edit auth-bug --description "Fixed authentication bug" --priority low

# Complete a task
todo complete auth-bug

# Delete a task
todo delete test-001
```

## 📖 Commands

### `add` - Create New Tasks
```bash
todo add <key> <description> [options]
```

**Options:**
- `--priority <level>` - Set priority (low, medium, high)
- `--status <status>` - Set status (not-started, in-progress, paused, completed, cancelled)
- `--due <date>` - Set due date in YYYYMMDD format

### `list` - Display Tasks
```bash
todo list [options]
```

**Options:**
- `--filter <text>` - Filter by description text
- `--status <status>` - Filter by task status
- `--verbose` - Show creation and update timestamps

### `edit` - Modify Tasks
```bash
todo edit <key> [options]
```

**Options:**
- `--description <text>` - Update task description
- `--priority <level>` - Update priority level
- `--status <status>` - Update task status
- `--due <date>` - Update due date (YYYYMMDD format)

### `complete` - Mark Tasks Complete
```bash
todo complete <key>
```

### `delete` - Remove Tasks
```bash
todo delete <key>
```

## 🎨 Task Status & Priority

### Status Levels
- **Not Started** - Task created but work hasn't begun
- **In Progress** - Task is currently being worked on
- **Paused** - Task has been temporarily paused
- **Completed** - Task has been finished successfully
- **Cancelled** - Task has been cancelled and won't be completed

### Priority Levels
- **Low** - Low priority tasks (green)
- **Medium** - Medium priority tasks (yellow) - *Default*
- **High** - High priority tasks (red)

## 💾 Data Storage

Tasks are stored in a human-readable TOML file:
- **Debug mode**: `.todo.toml` in current directory
- **Release mode**: `~/.todo.toml` in home directory

## 🔮 Future Features

- **Backup/Restore** - Automatic backups and data recovery
- **Remote Storage** - Multi-device sync and team collaboration
- **Notifications** - Due date alerts and reminders

---

**A simple tool for busy developers**