use async_graphql::{
    *,
    http::GraphiQLSource,
};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use super::user::UserObject;
pub struct Query;

#[Object]
impl Query {
    pub async fn user(&self, username: String) -> Result<Option<UserObject>> {
        todo!()
    }
}