use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::{
    entity::RecordDetailModel,
    error::Error,
};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct CsvRecord{
    pub timestamp: NaiveDateTime,
    pub comment: String,
    pub tag: String,
    pub count: i32,
}

impl TryFrom<RecordDetailModel> for CsvRecord{
    type Error = Error;
    fn try_from(model: RecordDetailModel) -> Result<Self, Self::Error> {
        todo!()
    }
}