use clap::Parser;
use todo_rusty::{
    add_task, delete_task, list_tasks, load_tasks_from_file, mark_task_as_completed,
    save_tasks_to_file,
};
mod db;
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

    // Load existing tasks
    let mut tasks = load_tasks_from_file().expect("Failed to load tasks");

    // Handle CLI input
    if let Some(description) = cli.add {
        add_task(description, &mut tasks);
    }

    if let Some(id) = cli.complete {
        mark_task_as_completed(id, &mut tasks);
    }

    if let Some(id) = cli.delete {
        delete_task(id, &mut tasks);
    }

    if cli.list {
        list_tasks(&tasks);
    }

    // Save tasks
    save_tasks_to_file(&tasks).expect("Failed to save tasks");
}
