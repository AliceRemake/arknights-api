use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use log::{error, info};
use service::{migration::operator::migrate, resource::{fs, get_local_resource_instance, get_remote_resource_instance, HOME}};

pub static ROUTE: &'static str = "/update";

pub async fn post(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, &'static str), (StatusCode, &'static str)> {
    let client_token = match payload["TOKEN"].as_str() {
        Some(token) => token,
        None => {
            let response = (StatusCode::BAD_REQUEST, "can not parse `TOKEN`");
            error!("[{}]{:?}", ROUTE, response);
            return Err(response);
        }
    };

    let server_token = match std::env::var("TOKEN") {
        Ok(token) => token,
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    };

    if client_token != server_token {
        let response = (StatusCode::UNAUTHORIZED, "`TOKEN` not correct");
        error!("[{}]{:?}", ROUTE, response);
        return Err(response);
    }

    let local_resource = get_local_resource_instance();
    let remote_resource = get_remote_resource_instance();

    let local_version = match local_resource.version() {
        Ok(version) => Some(version),
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    };

    let remote_version = match remote_resource.version().await {
        Ok(version) => Some(version),
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    };

    if local_version != remote_version {
        let local_path = format!(
            "{}/{}/{}",
            HOME.as_str(),
            local_resource.dist,
            local_resource.repo
        );

        if !fs::exists(&local_path)
            || (fs::is_dir(&local_path)
                && match fs::is_empty(&local_path) {
                    Ok(is_empty) => is_empty,
                    Err(err) => {
                        let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
                        error!("[{}]{:?}{}", ROUTE, response, err);
                        return Err(response);
                    }
                })
        {
            match local_resource.init() {
                Ok(_) => {}
                Err(err) => {
                    let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
                    error!("[{}]{:?}{}", ROUTE, response, err);
                    return Err(response);
                }
            }
        }

        match local_resource.pull(remote_resource) {
            Ok(_) => {}
            Err(err) => {
                let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
                error!("[{}]{:?}{}", ROUTE, response, err);
                return Err(response);
            }
        }
    }

    match migrate(&state.db).await {
        Ok(_) => {}
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    }

    let response = (StatusCode::OK, "Ok");
    info!("[{}]{:?}", ROUTE, response);
    Ok(response)
}
