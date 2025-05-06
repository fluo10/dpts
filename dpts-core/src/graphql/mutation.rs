use async_graphql::*;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn login(&self, username:String, password: String) -> Result<String> {
        todo!()
    }
}