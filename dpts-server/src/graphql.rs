use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{routing::get, Router};
use dpts_core::graphql::{
    Query,
    Mutation,
};

pub fn build_schema() -> Schema<Query, Mutation, EmptySubscription>{
    Schema::build(Query, Mutation, EmptySubscription).finish()
}

pub fn build_service() -> GraphQL<Schema<Query, Mutation, EmptySubscription>> {
    GraphQL::new(build_schema())
}