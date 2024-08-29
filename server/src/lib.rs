#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

pub use axum::{
    extract::{Json, Path, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
};

pub mod home;
pub mod migrate;
pub mod resource;
pub mod update;

pub mod enemy;
pub mod operator;
