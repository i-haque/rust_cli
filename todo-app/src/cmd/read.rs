use crate::db::crud::{read_all, read_one};
use cli_table::{WithTitle, print_stdout};
use inquire::{Text, error::InquireResult};

pub fn read_all_tasks() {
    println!();
    let tasks = read_all();
    if tasks.is_empty() {
        println!("Wohoo, no pending tasks!");
    } else {
        print_stdout(tasks.with_title()).unwrap();
    }
    println!();
}

pub fn read_a_task() -> InquireResult<()> {
    let task_id = Text::new("Task Id:").prompt()?;
    let tasks = read_one(&task_id);
    if tasks.is_empty() {
        println!("No task found with this id!");
    } else {
        print_stdout(tasks.with_title()).unwrap();
    }

    Ok(())
}
