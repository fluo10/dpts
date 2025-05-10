use async_graphql::*;
use chrono::{DateTime, FixedOffset};

#[derive(SimpleObject)]
pub struct PartialUser {
    pub id: Option<i32>,
    pub login_name: Option<String>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}

pub struct CreateUser {
    login_name: String,
    password: String,
}
pub struct ModifyUser {
    login_name: Option<String>,
    password: Option<String>,
}
