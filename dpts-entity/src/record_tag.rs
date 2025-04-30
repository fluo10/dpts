use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "record_tag")]
#[graphql(concrete(name = "RecordTag", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(indexed)]
    #[serde(skip_deserializing)]
    pub user_id: i32,
    #[sea_orm(indexed)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::record_detail::Entity")]
    RecordDetail,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::record_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecordDetail.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}