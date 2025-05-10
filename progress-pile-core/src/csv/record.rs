use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CsvRecord{
    pub timestamp: NaiveDateTime,
    pub comment: String,
    pub tag: String,
    pub count: i32,
}
