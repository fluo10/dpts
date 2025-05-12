use async_graphql::{
    *,
    http::GraphiQLSource,
};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use crate::{entity::{UserEntity, UserModel},};

pub struct Query;

#[Object]
impl Query {
    pub async fn user(&self, user_name: String) -> Result<Option<String>> {
        todo!()
    }
    pub async fn users(&self) -> Result<Vec<String>> {
        todo!()
    }
}