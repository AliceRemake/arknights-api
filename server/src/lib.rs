#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

pub use axum::{
    extract::{Json, Path, State},
    http::{StatusCode, HeaderMap, HeaderName, HeaderValue},
};

pub mod home;
pub mod update;

pub mod operator;
pub mod resource;
