use crate::db::crud::create;
use crate::tasks::{priority::Priority, status::Status, task::Task};
use inquire::{Select, Text, error::InquireResult};
use short_id::short_id_ordered;

pub fn create_new_task() -> InquireResult<()> {
    println!();
    let id = short_id_ordered();
    let title = Text::new("Title:").prompt()?;
    let priority = Select::new("Priority:", vec![Priority::Low, Priority::High]).prompt()?;
    let status = Status::Pending;
    create(Task {
        id,
        title,
        priority,
        status,
    });

    println!("New task created!\n");

    Ok(())
}
