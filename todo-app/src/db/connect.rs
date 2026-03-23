use sqlite3::Connection;
use std::fs;

pub fn connect_to_db() -> Connection {
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
            "CREATE TABLE IF NOT EXISTS tasks (id TEXT PRIMARY KEY, title TEXT, priority TEXT, status TEXT, created_at TEXT);",
        )
        .expect("uable to create a table!");
    connection
}
