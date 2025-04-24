use crate::label::Label;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Record {
    pub label: String,
    pub comment: String,
    pub count: u8,
    pub date: DateTime<Utc>,
}
