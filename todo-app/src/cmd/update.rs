use inquire::{Select, Text};

use crate::db::crud::{change_priority, id_exists, mark_done};

pub fn choose_option() {
    println!();
    let choice = Select::new(
        "What do you want to update?",
        vec!["Mark as completed", "Change priority"],
    )
    .prompt()
    .unwrap();
    if choice == "Mark as completed" {
        mark_as_completed();
    } else {
        change_task_priority();
    }
}

pub fn mark_as_completed() {
    let task_id = Text::new("Task Id:").prompt().unwrap();
    if id_exists(&task_id) {
        mark_done(&task_id);
        println!("Task with id: {} marked as completed!\n", task_id);
    } else {
        println!("Task with id: {} doesn't exist!\n", task_id);
    }
}

pub fn change_task_priority() {
    let task_id = Text::new("Task Id:").prompt().unwrap();
    if id_exists(&task_id) {
        change_priority(&task_id);
        println!("Priority changed for task with id: {}!\n", task_id);
    } else {
        println!("Task with id: {} doesn't exist!\n", task_id);
    }
}
