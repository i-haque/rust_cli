use crate::db::connect::connect_to_db;
use crate::tasks::task::Task;

pub fn create(new_task: Task) {
    let conn = connect_to_db();
    let query = format!(
        "INSERT INTO tasks (id, title, priority, status, created_at) VALUES ('{}', '{}', '{}', '{}', '{}');",
        new_task.id,
        new_task.title,
        new_task.priority.to_string(),
        new_task.status.to_string(),
        new_task.created_at.to_string()
    );
    conn.execute(query).unwrap();
}
