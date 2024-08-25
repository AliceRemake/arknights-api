use crate::*;
use log;

pub static ROUTE: &'static str = "/operator/query";

pub async fn get(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<Vec<entity::operator::Model>>), (StatusCode, &'static str)> {
    let name = match payload["name"].as_str() {
        Some(token) => token.to_owned(),
        None => {
            let response = (StatusCode::BAD_REQUEST, "can not parse `name`");
            log::error!("[{}]{:?}", ROUTE, response);
            return Err(response);
        }
    };

    let response = match service::operator::Query::find_by_name(&state.db, name).await {
        Ok(response) => response,
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            log::error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    };

    log::info!("[{}]{:?}", ROUTE, response);
    Ok((StatusCode::OK, Json(response)))
}
