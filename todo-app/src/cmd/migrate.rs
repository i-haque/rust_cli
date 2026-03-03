use crate::db::crud::migrate_db;

pub fn migrate_data() {
    println!();
    migrate_db();
    println!("Existing data migrated!\n");
}
