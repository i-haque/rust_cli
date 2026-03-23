use crate::db::connect::connect_to_db;
use crate::tasks::{priority::Priority, status::Status, task::Task};
use sqlite3::Value;
use time::{
    Date, OffsetDateTime,
    macros::{date, format_description},
};

pub fn read_all() -> Vec<Task> {
    let conn = connect_to_db();
    let mut cursor = conn
        .prepare("SELECT * FROM tasks")
        .expect("unable to read all data from table!")
        .cursor();

    let mut tasks = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        let id: String = row[0].as_string().unwrap().to_owned();
        let title: String = row[1].as_string().unwrap().to_owned();
        let priority: Priority = row[2].as_string().unwrap().parse().unwrap();
        let status: Status = row[3].as_string().unwrap().parse().unwrap();
        let created_at: Date = Date::parse(
            row[4]
                .as_string()
                .unwrap_or(&date!(9999 - 12 - 31).to_string()),
            format_description!("[year]-[month]-[day]"),
        )
        .unwrap();
        let task = Task {
            id,
            title,
            priority,
            status,
            created_at,
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
        let id: String = row[0].as_string().unwrap().to_owned();
        let title: String = row[1].as_string().unwrap().to_owned();
        let priority: Priority = row[2].as_string().unwrap().parse().unwrap();
        let status: Status = row[3].as_string().unwrap().parse().unwrap();
        let created_at: Date = Date::parse(
            row[4]
                .as_string()
                .unwrap_or(&date!(9999 - 12 - 31).to_string()),
            format_description!("[year]-[month]-[day]"),
        )
        .unwrap_or(OffsetDateTime::now_utc().date());
        let task = Task {
            id,
            title,
            priority,
            status,
            created_at,
        };
        tasks.push(task);
    }
    tasks
}
