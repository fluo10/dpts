use async_graphql::*;
use super::user::UserObject;
struct Query;

#[Object]
impl Query {
    async fn user(&self, username: String) -> Result<Option<UserObject>> {
        todo!()
    }
}