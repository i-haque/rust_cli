use crate::tasks::priority::Priority;
use crate::tasks::status::Status;
use cli_table::Table;
use time::Date;

#[allow(dead_code)]
#[derive(Debug, Table)]
pub struct Task {
    #[table(title = "ID")]
    pub id: String,
    #[table(title = "Title")]
    pub title: String,
    #[table(title = "Priority")]
    pub priority: Priority,
    #[table(title = "Status")]
    pub status: Status,
    #[table(title = "Created At")]
    pub created_at: Date,
}
