#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

pub use axum::{
    extract::{Json, State},
    http::StatusCode,
};

pub mod home;
pub mod update;
pub mod operator;
