// this file contain functions to save the todo's in json file
use std::{
    fs::File,
    io::{self, BufReader, Write},
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: usize,
    description: String,
    completed: bool,
}

pub fn save_tasks_to_file(tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = File::create("tasks.json")?;
    let json = serde_json::to_string(tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_tasks_from_file() -> io::Result<Vec<Task>> {
    let file = File::open("tasks.json").unwrap_or_else(|_| File::create("tasks.json").unwrap());
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

pub fn add_task(description: String, tasks: &mut Vec<Task>) {
    if let Some(last_task) = tasks.last() {
        let task = Task {
            id: last_task.id + 1,
            description,
            completed: false,
        };
        tasks.push(task);
    } else {
        let task = Task {
            id: tasks.len() + 1,
            description,
            completed: false,
        };
        tasks.push(task);
    }
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        return println!("seems that you don't have any task to list :>)");
    }
    for task in tasks {
        println!(
            "{}: {} [{}]",
            task.id,
            task.description,
            if task.completed { "Done" } else { "Pending" }
        );
    }
}

pub fn mark_task_as_completed(id: usize, tasks: &mut Vec<Task>) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true
    }
}

pub fn delete_task(id: usize, tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("Seems that you don't have any tasks to delete :>)");
        return;
    }

    let original_len = tasks.len();
    tasks.retain(|task| task.id != id);

    if tasks.len() == original_len {
        println!(
            "Seems that you don't have any task with this ID to delete :>) ID: {}",
            id
        );
    }
}
