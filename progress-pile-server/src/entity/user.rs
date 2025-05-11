use async_graphql::SimpleObject;
use chrono::{DateTime, FixedOffset, Local,};
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};
use crate::error::Error;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, SimpleObject, Deserialize)]
#[sea_orm(table_name = "user")]
#[graphql(concrete(name = "User", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(unique, indexed)]
    pub login_name: String,
    pub password_hash: String,
    #[sea_orm(indexed)]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(indexed)]
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::access_token::Entity")]
    AccessToken,
    #[sea_orm(has_many = "super::progress_category::Entity")]
    ProgressCategory,
    #[sea_orm(has_many = "super::progress_entry::Entity")]
    ProgressEntry,
}

impl Related<super::access_token::Entity> for Model {
    fn to() -> RelationDef {
        Relation::AccessToken.def()
    }
}

impl Related<super::progress_category::Entity> for Model {
    fn to() -> RelationDef {
        Relation::ProgressCategory.def()
    }
}

impl Related<super::progress_entry::Entity> for Model {
    fn to() -> RelationDef {
        Relation::ProgressEntry.def()
    }
}

impl ActiveModel {
    pub fn new() -> Self {
        let timestamp = Local::now().fixed_offset();
        Self { 
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            ..Default::default()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

