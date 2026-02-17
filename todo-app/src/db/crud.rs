use sqlite3::{self, Connection, Value};
use std::fs;

use crate::tasks::{priority::Priority, status::Status, task::Task};

fn connect_to_db() -> Connection {
    // Initialize DB -> Creates a 'data' folder and a todo.db file in the executable's directory
    let curr_path = app_path::app_path!("data");
    if !curr_path.exists() {
        fs::create_dir(curr_path.to_owned()).expect("unable to create folder!");
    }

    let db_path = curr_path.join("todo.db");
    if !db_path.exists() {
        fs::File::create_new(db_path.to_owned()).expect("unable to create db file!");
    }

    // Connects to the database file
    let connection = sqlite3::open(db_path).expect("unable to connect to the database!");
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS tasks (id TEXT PRIMARY KEY, title TEXT, priority TEXT, status TEXT);",
        )
        .expect("uable to create a table!");
    connection
}

pub fn id_exists(id: &str) -> bool {
    let conn = connect_to_db();
    let mut cursor = conn
        .prepare("SELECT * FROM tasks WHERE id = ?")
        .unwrap()
        .cursor();
    cursor.bind(&[Value::String(id.to_owned())]).unwrap();

    let mut rows = 0;
    while let Some(_) = cursor.next().unwrap() {
        rows += 1;
    }

    if rows == 0 {
        return false;
    }
    true
}

// CREATE

pub fn create(new_task: Task) {
    let conn = connect_to_db();
    let query = format!(
        "INSERT INTO tasks (id, title, priority, status) VALUES ('{}', '{}', '{}', '{}');",
        new_task.id,
        new_task.title,
        new_task.priority.to_string(),
        new_task.status.to_string()
    );
    conn.execute(query).unwrap();
}

// READ

pub fn read_all() -> Vec<Task> {
    let conn = connect_to_db();
    let mut cursor = conn
        .prepare("SELECT * FROM tasks")
        .expect("unable to read all data from table!")
        .cursor();

    let mut tasks = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        let id = row[0].as_string().unwrap().to_string();
        let title = row[1].as_string().unwrap().to_string();
        let priority: Priority = row[2].as_string().unwrap().parse().unwrap();
        let status: Status = row[3].as_string().unwrap().parse().unwrap();
        let task = Task {
            id,
            title,
            priority,
            status,
        };
        tasks.push(task);
    }
    tasks
}

pub fn read_one(id: &str) -> Vec<Task> {
    let conn = connect_to_db();
    let mut cursor = conn
        .prepare("SELECT * FROM tasks WHERE id = ?")
        .unwrap()
        .cursor();
    cursor.bind(&[Value::String(id.to_owned())]).unwrap();

    let mut tasks = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        let id = row[0].as_string().unwrap().to_owned();
        let title = row[1].as_string().unwrap().to_owned();
        let priority: Priority = row[2].as_string().unwrap().parse().unwrap();
        let status: Status = row[3].as_string().unwrap().parse().unwrap();
        let task = Task {
            id,
            title,
            priority,
            status,
        };
        tasks.push(task);
    }
    tasks
}

// UPDATE

pub fn mark_done(id: &str) {
    let conn = connect_to_db();
    let query = format!(
        "UPDATE tasks SET status = '{}' WHERE id = '{}';",
        Status::Completed.to_string(),
        id
    );
    conn.execute(query).unwrap();
}

pub fn change_priority(id: &str) {
    let conn = connect_to_db();
    let mut cursor = conn
        .prepare("SELECT priority from tasks WHERE id = ?;")
        .unwrap()
        .cursor();
    cursor.bind(&[Value::String(id.to_owned())]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        let curr_priority: Priority = row[0].as_string().unwrap().parse().unwrap();
        if curr_priority == Priority::Low {
            let query = format!(
                "UPDATE tasks SET priority = '{}' WHERE id = '{}';",
                Priority::High.to_string(),
                id
            );
            conn.execute(query).unwrap();
        } else {
            let query = format!(
                "UPDATE tasks SET priority = '{}' WHERE id = '{}';",
                Priority::Low.to_string(),
                id
            );
            conn.execute(query).unwrap();
        }
    }
}

// DELETE

pub fn delete_all() {
    let conn = connect_to_db();
    let query = String::from("DELETE FROM tasks");
    conn.execute(query).unwrap();
}

pub fn delete_one(id: &str) {
    let conn = connect_to_db();
    let query = format!("DELETE FROM tasks WHERE id = '{}';", id);
    conn.execute(query).unwrap();
}
