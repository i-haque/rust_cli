use crate::db::connect::connect_to_db;
use crate::tasks::{priority::Priority, status::Status};
use sqlite3::Value;

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
