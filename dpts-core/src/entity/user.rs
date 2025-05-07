use async_graphql::SimpleObject;
use chrono::{DateTime, FixedOffset,};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::error::Error;

use crate::DATABASE_CONNECTION;

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

impl Entity {
    pub async fn find_by_name(user_name: &str) -> Result<Option<Model>, Error> {
        Ok(Entity::find()
            .filter(Column::LoginName.contains(user_name))
            .one(DATABASE_CONNECTION.get().unwrap())
            .await?
        )
    }
    pub async fn find_all() -> Result<Vec<Model>, Error> {
        Ok(Entity::find()
            .all(DATABASE_CONNECTION.get().unwrap())
            .await?
        )
    }
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
impl ActiveModel {
    pub async fn create_user(login_name: &str, password: &str) -> Result<Model, Error>{
        todo!()
    }
}
impl ActiveModelBehavior for ActiveModel {}

