use crate::*;

use log;

pub static ROUTE: &'static str = "/";

pub async fn get() -> Result<(StatusCode, &'static str), (StatusCode, &'static str)> {
    let response = (StatusCode::OK, "hello, arknights-api!");
    log::info!("[{}]{:?}", ROUTE, response);
    Ok(response)
}
