use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
 


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CsvRecord{
    pub timestamp: NaiveDateTime,
    pub comment: String,
    pub tag: String,
    pub count: i32,
}

impl TryFrom<crate::entity::RecordDetailModel> for CsvRecord{
    type Error = crate::error::Error;
    fn try_from(model: crate::entity::RecordDetailModel) -> Result<Self, Self::Error> {
        todo!()
    }
}