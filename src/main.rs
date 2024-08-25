use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use env_logger;
use error::Error;
use log::info;
use sea_orm::{Database, DatabaseConnection};
use server::state::AppState;
use service::resource::{fs, get_local_resource_instance, get_remote_resource_instance, HOME};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();

    info!("updating resource on boost");
    let local_resource = get_local_resource_instance();
    let remote_resource = get_remote_resource_instance();

    let local_version = match local_resource.version() {
        Ok(version) => Some(version),
        Err(_) => None,
    };

    let remote_version = match remote_resource.version().await {
        Ok(version) => Some(version),
        Err(_) => None,
    };

    if local_version != remote_version {
        let local_path = format!(
            "{}/{}/{}",
            HOME.as_str(),
            local_resource.dist,
            local_resource.repo
        );

        if !fs::exists(&local_path) || (fs::is_dir(&local_path) && fs::is_empty(&local_path)?) {
            local_resource.init()?;
        }

        local_resource.pull(remote_resource)?;
    }
    info!("updated resource");

    let url: String = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("can not find \"DATABASE_URL\" in \".env\" file"),
    };

    info!("connecting to database {}", &url);
    let db: DatabaseConnection = match Database::connect(&url).await {
        Ok(db) => db,
        Err(_) => panic!("can not connect to database"),
    };
    info!("connected to database {}", &url);

    let state: AppState = AppState { db };

    let app: Router = Router::new()
        .route(server::home::ROUTE, get(server::home::get))
        .route(server::update::ROUTE, post(server::update::post))
        .route(server::operator::query::ROUTE, get(server::operator::query::get))
        .with_state(state);

    const ADDRESS: &'static str = "127.0.0.1:3000";

    let listener: tokio::net::TcpListener = match tokio::net::TcpListener::bind(ADDRESS).await {
        Ok(listener) => listener,
        Err(_) => panic!("can not bind {}", ADDRESS),
    };

    info!("arknights-api is running at {}", &ADDRESS);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
