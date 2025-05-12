use async_graphql::*;

use crate::{auth::try_hash_password, entity::UserActiveModel};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn login(&self, username:String, password: String) -> Result<String> {
        todo!()
    }
    async fn create_user(&self, username:String, password: String) -> Result<String> {
        todo!()
    }
}