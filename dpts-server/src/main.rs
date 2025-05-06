use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, *};
use async_graphql_axum::GraphQL;
use dpts_core::graphql::{
    Mutation,
    Query,
};
use dpts_server::{build_app, Args};
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use clap::Parser;



#[tokio::main]
async fn main() {
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        crate::build_app()
    ).await.unwrap()
}


