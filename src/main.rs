use axum::routing::{get, post};

const ADDRESS: &'static str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("updating local resource");
    if service::resource::need_update().await? {
        service::resource::update_local_resource().await?;
    }
    log::info!("updated local resource");

    let url: String = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("can not find \"DATABASE_URL\" in \".env\" file"),
    };

    log::info!("connecting to database {}", &url);
    let db = match sea_orm::Database::connect(&url).await {
        Ok(db) => db,
        Err(_) => panic!("can not connect to database"),
    };
    log::info!("connected to database {}", &url);

    let state = server::AppState { db };

    let app = axum::Router::new()
        .route(server::home::ROUTE, get(server::home::get))
        .route(server::update::ROUTE, post(server::update::post))
        .route(
            server::operator::query::ROUTE,
            get(server::operator::query::get),
        )
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind(ADDRESS).await {
        Ok(listener) => listener,
        Err(_) => panic!("can not bind {}", ADDRESS),
    };

    log::info!("arknights-api is running at {}", &ADDRESS);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
