mod args;
mod auth;
mod config;
pub mod error;
pub mod graphql;

pub use args::Args;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{
    GraphQL,

};
use axum::{routing::get, Router};
use crate::graphql::build_service;
use dpts_core::entity as entity;


pub fn build_app() -> Router {
    let router = Router::new()
        .route_service("/graphql", build_service());
    router
}