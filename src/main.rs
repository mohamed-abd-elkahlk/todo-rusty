use clap::Parser;
mod db;
use db::{add_task, delete_task, initialize_db, list_tasks, mark_task_as_completed};

#[derive(Parser)]
#[command(name = "Todo App")]
#[command(about = "A simple CLI-based todo app in Rust")]
struct Cli {
    #[arg(short, long)]
    add: Option<String>,
    #[arg(short, long)]
    delete: Option<usize>,
    #[arg(short, long)]
    complete: Option<usize>,
    #[arg(short, long)]
    list: bool,
}

fn main() {
    let cli = Cli::parse();

    // Initialize the database connection
    let conn = initialize_db().expect("Failed to initialize database");

    // Handle CLI input
    if let Some(description) = cli.add {
        add_task(&conn, description).expect("Failed to add task");
    }

    if let Some(id) = cli.complete {
        mark_task_as_completed(&conn, id).expect("Failed to mark task as completed");
    }

    if let Some(id) = cli.delete {
        delete_task(&conn, id).expect("Failed to delete task");
    }

    if cli.list {
        list_tasks(&conn).expect("Failed to list tasks");
    }
}
