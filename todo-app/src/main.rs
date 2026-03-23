use clap::{Parser, Subcommand};
use inquire::error::InquireResult;

mod db;
mod tasks;

mod cmd;
use cmd::{create, delete, read, update};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds a new task
    CreateTask,
    /// Get all the tasks
    GetTasks,
    /// Get a particular task with an ID
    GetTask,
    /// Updates a particular task
    UpdateTask,
    /// Delete all tasks
    DeleteTasks,
    /// Delete a particular task with an ID
    DeleteTask,
}

fn main() -> InquireResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::CreateTask) => {
            create::create_new_task()?;
        }
        Some(Commands::GetTasks) => {
            read::read_all_tasks();
        }
        Some(Commands::GetTask) => {
            read::read_a_task()?;
        }
        Some(Commands::UpdateTask) => {
            update::choose_option();
        }
        Some(Commands::DeleteTasks) => {
            delete::delete_all_tasks();
        }
        Some(Commands::DeleteTask) => {
            delete::delete_a_task();
        }
        None => println!("Use --help for more info!"),
    }

    Ok(())
}
