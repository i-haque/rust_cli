use clap::ValueEnum;
use std::fmt;
use std::str::FromStr;

#[derive(ValueEnum, Debug, Clone)]
pub enum Status {
    Pending,
    Completed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Completed => write!(f, "completed"),
        }
    }
}

#[derive(Debug)]
pub struct ParseStatusError;

impl FromStr for Status {
    type Err = ParseStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" | "Pending" => Ok(Status::Pending),
            "completed" | "Completed" => Ok(Status::Completed),
            _ => Err(ParseStatusError),
        }
    }
}
