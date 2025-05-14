pub mod cli;
pub mod auth;
pub mod config;
pub mod entity;
pub mod global;
pub mod graphql;
pub use progress_pile_core::error;

pub use cli::Cli;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{
    GraphQL,

};
use axum::{routing::get, Router};
use crate::graphql::build_service;


pub fn build_app() -> Router {
    let router = Router::new()
        .route_service("/graphql", build_service());
    router
}