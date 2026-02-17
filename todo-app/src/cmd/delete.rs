use inquire::{Confirm, Text};

use crate::db::crud::{delete_all, delete_one, id_exists};

pub fn delete_all_tasks() {
    println!();
    let confirmation = Confirm::new("Are you sure?")
        .with_default(false)
        .with_help_message("All tasks will be erased. This action cannot be undone!")
        .prompt()
        .unwrap();

    if confirmation {
        delete_all();
        println!("All tasks deleted!");
    }
    println!();
}

pub fn delete_a_task() {
    println!();
    let task_id = Text::new("Task Id:").prompt().unwrap();
    if id_exists(&task_id) {
        delete_one(&task_id);
        println!("Task with id: {} deleted!\n", task_id);
    } else {
        println!("Task with id: {} doesn't exist!\n", task_id);
    }
}
