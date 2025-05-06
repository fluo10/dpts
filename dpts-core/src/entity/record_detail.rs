use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "record_detail")]
#[graphql(concrete(name = "RecordDetail", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub record_header_id: i32,
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub record_tag_id: i32,
    pub count: i32,
}

#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::record_header::Entity",
        from = "Column::RecordHeaderId",
        to = "super::record_header::Column::Id"
    )]
    RecordHeader,
    #[sea_orm(
        belongs_to = "super::record_tag::Entity",
        from = "Column::RecordTagId",
        to = "super::record_tag::Column::Id"
    )]
    RecordTag,
}

impl Related<super::record_header::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecordHeader.def()
    }
}

impl Related<super::record_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecordTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}