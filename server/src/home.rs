use axum::{extract::State, http::StatusCode};
use log;

use crate::state::AppState;

pub static ROUTE: &'static str = "/";

pub async fn get(
    State(_): State<AppState>,
) -> Result<(StatusCode, &'static str), (StatusCode, &'static str)> {
    let response = (StatusCode::OK, "hello, arknights-api!");
    log::info!("[{}]{:?}", ROUTE, response);
    Ok(response)
}
