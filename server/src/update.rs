use crate::*;

use log;

pub static ROUTE: &'static str = "/update";

pub async fn post(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, &'static str), (StatusCode, &'static str)> {
    let client_token = match payload["TOKEN"].as_str() {
        Some(token) => token,
        None => {
            let response = (StatusCode::BAD_REQUEST, "bad parameter `TOKEN`");
            log::error!("[{}]{:?}", ROUTE, response);
            return Err(response);
        }
    };

    let server_token = match std::env::var("TOKEN") {
        Ok(token) => token,
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            log::error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    };

    if client_token != server_token {
        let response = (StatusCode::UNAUTHORIZED, "`TOKEN` not correct");
        log::error!("[{}]{:?}", ROUTE, response);
        return Err(response);
    }

    log::info!("[{}]{}", ROUTE, "updating local resource");
    match service::resource::need_update().await {
        Ok(need_update) => {
            if need_update {
                match service::resource::update_local_resource().await {
                    Ok(_) => log::info!("[{}]{}", ROUTE, "updated local resource"),
                    Err(err) => {
                        let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
                        log::error!("[{}]{:?}{}", ROUTE, response, err);
                        return Err(response);
                    }
                }
            }
        }
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            log::error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    }

    log::info!("[{}]{}", ROUTE, "migrating database");
    match service::migration::migrate(&state.db).await {
        Ok(_) => log::info!("[{}]{}", ROUTE, "migrated database"),
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            log::error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    }

    let response = (StatusCode::OK, "Ok");
    log::info!("[{}]{:?}", ROUTE, response);
    Ok(response)
}
