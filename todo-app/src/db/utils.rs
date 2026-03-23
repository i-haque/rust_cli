use crate::db::connect::connect_to_db;
use sqlite3::Value;

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
