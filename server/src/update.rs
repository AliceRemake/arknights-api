use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::state::AppState;

pub async fn post(State(_): State<AppState>, Json(payload): Json<serde_json::Value>) -> Result<&'static str, (StatusCode, &'static str)> {
    println!("{}", payload["token"].as_str().unwrap());
    Ok("123")
}
