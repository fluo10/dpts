use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Record {
    pub comment: String,
    pub date: DateTime<Utc>,
    pub achievements: HashMap<String, i8>,
}
