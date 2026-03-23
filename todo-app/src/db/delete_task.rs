use crate::db::connect::connect_to_db;

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
