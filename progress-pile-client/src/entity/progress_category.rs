use core::time;

use chrono::Local;
use sea_orm::entity::{
    *,
    prelude::*
};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "progress_category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub name: String,
    #[sea_orm(indexed)]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    
    #[sea_orm(has_many = "super::ProgressEntryEntity")]
    ProgressEntry,
    }
    
impl Related<super::ProgressEntryEntity> for Entity {
    fn to() -> RelationDef {
        Relation::ProgressEntry.def()
    }
    
}
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new() -> Self {
        let timestamp: DateTimeWithTimeZone = Local::now().fixed_offset();
        Self{
            id: Set(Uuid::new_v4()),
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            ..Default::default()
        }
    }
}