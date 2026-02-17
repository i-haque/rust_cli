use crate::tasks::priority::Priority;
use crate::tasks::status::Status;
use cli_table::{Table, format::Justify};

#[allow(dead_code)]
#[derive(Debug, Table)]
pub struct Task {
    #[table(title = "ID", justify = "Justify::Right")]
    pub id: String,
    #[table(title = "Title")]
    pub title: String,
    #[table(title = "Priority")]
    pub priority: Priority,
    #[table(title = "Status")]
    pub status: Status,
}
