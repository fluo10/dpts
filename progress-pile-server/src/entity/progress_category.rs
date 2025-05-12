use async_graphql::*;
use chrono::Local;
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,)]
#[sea_orm(table_name = "progress_category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
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
    #[sea_orm(has_many = "super::progress_entry::Entity")]
    ProgressEntry,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}
    
impl Related<super::progress_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProgressEntry.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
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