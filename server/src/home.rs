use axum::{extract::State, http::StatusCode};

use crate::state::AppState;

pub async fn get(State(_): State<AppState>) -> Result<&'static str, (StatusCode, &'static str)> {
    Ok("hello, arknights-api!")
}
