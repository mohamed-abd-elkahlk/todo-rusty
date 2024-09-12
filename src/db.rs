// this file contain functions to save the todo's using sqlite

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

pub fn initialize_db() -> Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                  id          INTEGER PRIMARY KEY,
                  description TEXT NOT NULL,
                  completed   BOOLEAN NOT NULL
                  )",
        [],
    )?;
    Ok(conn)
}

pub fn add_task(conn: &Connection, description: String) -> Result<()> {
    conn.execute(
        "INSERT INTO task (description, completed) VALUES (?1, ?2)",
        params![description, false],
    )?;
    println!("Task added: {}", description);
    Ok(())
}

pub fn delete_task(conn: &Connection, id: usize) -> Result<()> {
    let affected_rows = conn.execute("DELETE FROM task WHERE id = ?1", params![id])?;
    if affected_rows == 0 {
        println!("Task with ID {} not found.", id);
    } else {
        println!("Task with ID {} deleted.", id);
    }
    Ok(())
}

pub fn list_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM task")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }

    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for task in tasks {
            println!(
                "{}: {} [{}]",
                task.id,
                task.description,
                if task.completed { "Done" } else { "Pending" }
            );
        }
    }
    Ok(())
}

pub fn mark_task_as_completed(conn: &Connection, id: usize) -> Result<()> {
    let affected_rows = conn.execute(
        "UPDATE task SET completed = ?1 WHERE id = ?2",
        params![true, id],
    )?;
    if affected_rows == 0 {
        println!("Task with ID {} not found.", id);
    } else {
        println!("Task with ID {} marked as completed.", id);
    }
    Ok(())
}
