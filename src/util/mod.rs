use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

pub mod htmx;
pub mod pagination;
pub mod rbac;
pub mod validation;

// String Utilities

pub trait StringExt {
    fn is_whitespace_or_empty(&self) -> bool;
}

impl StringExt for String {
    fn is_whitespace_or_empty(&self) -> bool {
        self.trim().is_empty()
    }
}

impl StringExt for str {
    fn is_whitespace_or_empty(&self) -> bool {
        self.trim().is_empty()
    }
}

pub fn parse_no_seconds<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M").map_err(serde::de::Error::custom)
}
