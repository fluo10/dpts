use async_graphql::SimpleObject;
use chrono::{DateTime, FixedOffset,};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::record_header::Entity")]
    RecordHeader,
    #[sea_orm(has_many = "super::record_tag::Entity")]
    RecordTag,
}

impl Related<super::record_header::Entity> for Model {
    fn to() -> RelationDef {
        Relation::RecordHeader.def()
    }
}

impl Related<super::record_tag::Entity> for Model {
    fn to() -> RelationDef {
        Relation::RecordTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}