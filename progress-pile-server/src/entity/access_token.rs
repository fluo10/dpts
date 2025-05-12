use core::time;

use async_graphql::*;
use chrono::Local;
use sea_orm::entity::{
    Set,
    prelude::*
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,)]
#[sea_orm(table_name = "access_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub user_id: i32,
    #[sea_orm(indexed)]
    pub token_value: String,
    pub note: String,
    #[sea_orm(indexed)]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(indexed)]
    pub expired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl ActiveModel {
    pub fn new() -> Self {
        let timestamp = Local::now().fixed_offset();
        Self{
            note: Set("".to_string()),
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            ..Default::default()
        }
    }
}
impl ActiveModelBehavior for ActiveModel {}