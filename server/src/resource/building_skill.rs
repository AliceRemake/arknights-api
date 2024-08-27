use crate::*;
use service::resource::*;

use log;

pub static ROUTE: &'static str = "/resource/building_skill/:icon";

pub async fn get(
    Path(icon): Path<String>,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), (StatusCode, &'static str)> {
    let path = format!(
        "{}/{}/{}/{}/{}",
        HOME.as_str(),
        LOCAL_RESOURCE.dist,
        LOCAL_RESOURCE.repo,
        "building_skill",
        format!("{}.png", icon)
    );

    if !fs::exists(&path) {
        let response = (StatusCode::NOT_FOUND, "not found");
        log::error!("[{}]{:?}{}", ROUTE, response, icon);
        return Err(response);
    }

    let raw_data = match std::fs::read(path) {
        Ok(raw_data) => raw_data,
        Err(err) => {
            let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
            log::error!("[{}]{:?}{}", ROUTE, response, err);
            return Err(response);
        }
    };

    let mut header = HeaderMap::new();

    header.insert("Content-Type", HeaderValue::from_static("image"));
    header.insert(
        "Content-Disposition",
        // HeaderValue::from_static("image"),
        match HeaderValue::from_str(&format!("attachment; filename=\"{}.png\"", icon)) {
            Ok(value) => value,
            Err(err) => {
                let response = (StatusCode::INTERNAL_SERVER_ERROR, "");
                log::error!("[{}]{:?}{}", ROUTE, response, err);
                return Err(response);
            }
        },
    );
    header.insert("Content-Length", HeaderValue::from(raw_data.len()));

    let response = (StatusCode::OK, header, raw_data);
    log::info!("[{}]{}", ROUTE, icon);
    Ok(response)
}
