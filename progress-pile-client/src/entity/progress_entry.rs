use async_graphql::*;
use chrono::Local;
use sea_orm::entity::{
    *, 
    prelude::*
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,)]
#[sea_orm(table_name = "progress_entry")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub progress_category_id: Uuid,
    #[sea_orm(indexed)]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(indexed)]
    pub progressed_at: DateTimeWithTimeZone,
    pub quantity: i32,
    pub note: String,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {

    #[sea_orm(
        belongs_to = "super::ProgressCategoryEntity",
        from = "Column::ProgressCategoryId",
        to = "super::ProgressCategoryColumn::Id"
    )]
    ProgressCategory,
}


impl Related<super::progress_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProgressCategory.def()
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
            progressed_at: Set(timestamp),
            quantity: Set(1),
            note: Set("".to_string()),
            ..Default::default()
        }
    }
}