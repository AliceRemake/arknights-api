use crate::*;
use log;

pub static ROUTE: &'static str = "/operator/:name";

pub async fn get(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<(StatusCode, Json<Vec<entity::operator::Model>>), (StatusCode, &'static str)> {
    let response = match service::operator::Query::find_by_name(&state.db, name.clone()).await {
        Ok(response) => response,
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            log::error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    };

    let response = (StatusCode::OK, Json(response));
    log::info!("[{}]{:?}", ROUTE, name);
    Ok(response)
}
