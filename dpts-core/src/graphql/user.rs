use async_graphql::*;
use chrono::{DateTime, FixedOffset};


pub struct UserObject {
    pub id: i32,
    pub login_name: Option<String>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[Object]
impl UserObject {
    async fn id(&self) -> i32 {
        todo!()
    }
    async fn login_name(&self) -> Option<String> {
        todo!()
    }
    async fn created_at(&self) -> Option<DateTime<FixedOffset>> {
        todo!()
    }
    async fn updated_at(&self) -> Option<DateTime<FixedOffset>> {
        todo!()
    }
}

