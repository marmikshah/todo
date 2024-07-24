# todo: A simple command-line task manager built with Rust and SQLite.

[![Code Quality](https://github.com/marmikshah/todo/actions/workflows/code-quality.yml/badge.svg?branch=main)](https://github.com/marmikshah/todo/actions/workflows/code-quality.yml)
[![Test](https://github.com/marmikshah/todo/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/marmikshah/todo/actions/workflows/test.yml)

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)

## Introduction

todo is a lightweight and efficient command-line task manager that helps you keep track of your to-do list. Built with Rust for performance and SQLite for data storage, todo is a reliable tool for managing your tasks.

## Features

- Add tasks
- Remove tasks
- Update tasks
- Mark tasks as complete

## Installation

1. **Clone the repository:**

    ```sh
    git clone https://github.com/marmikshah/todo.git
    cd todo
    ```

2. **Build the project:**

    ```sh
    cargo build --release
    cd target/release/
    ```
 

## Usage

- **Initialize paths**

    ```sh
    ./todo init
    ```


- **Adding a task:**

    ```sh
    ./todo add 'Buy groceries'
    ```

- **List all tasks**

    ```sh
    ./todo list
    ```

- **Removing a task:**

    ```sh
    ./todo remove 1
    ```

- **Completing a task:**

    ```sh
    ./todo complete 1
    ```

## Configuration

The following environment vars can be used to modify the behaviour

- TODO_DATASTORE_DIR: Sets the directory to store data. Defaults to `/home/$(whoami)` 
- TODO_LOG_LEVEL: Logging level. Defaults to `info`


