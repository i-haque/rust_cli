use clap::ValueEnum;
use std::fmt;
use std::str::FromStr;

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
        }
    }
}

#[derive(Debug)]
pub struct ParsePriorityError;

impl FromStr for Priority {
    type Err = ParsePriorityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" | "Low" => Ok(Priority::Low),
            "high" | "High" => Ok(Priority::High),
            _ => Err(ParsePriorityError),
        }
    }
}
