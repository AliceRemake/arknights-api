use axum::{routing::get, Router};
use dotenv::dotenv;
use env_logger;
use sea_orm::{Database, DatabaseConnection};

use std::env;

use server::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let url: String = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("can not find \"DATABASE_URL\" in \".env\" file"),
    };

    let connection: DatabaseConnection = match Database::connect(url).await {
        Ok(connection) => connection,
        Err(_) => panic!("can not connect to database"),
    };

    let state: AppState = AppState { connection };

    let app: Router = Router::new()
        .route("/", get(server::home::get))
        .with_state(state);

    const ADDRESS: &'static str = "127.0.0.1:3000";

    let listener: tokio::net::TcpListener = match tokio::net::TcpListener::bind(ADDRESS).await {
        Ok(listener) => listener,
        Err(_) => panic!("can not bind {}", ADDRESS),
    };

    axum::serve(listener, app).await.unwrap();
}
