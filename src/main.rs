use axum::routing::{get, post};

const ADDRESS: &'static str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

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

    // DEBUG
    // log::info!("migrating database");
    // service::migration::migrate(&db).await?;
    // log::info!("migrated database");

    match service::resource::need_update().await {
        Ok(need_update) => {
            if need_update {
                log::info!("updating local resource");
                service::resource::update_local_resource().await?;
                log::info!("updated local resource");
                log::info!("migrating database");
                service::migration::migrate(&db).await?;
                log::info!("migrated database");
            } else {
                log::info!("local resource already up to date");
            }
        }
        Err(_) => {
            log::warn!("can not get latest version now, skip");
        }
    };

    let state = server::AppState { db };

    let app = axum::Router::new()
        .route(server::home::ROUTE, get(server::home::get))
        .route(server::update::ROUTE, post(server::update::post))
        .route(server::migrate::ROUTE, post(server::migrate::post))
        .route(
            server::operator::query::ROUTE,
            get(server::operator::query::get),
        )
        .route(server::enemy::query::ROUTE, get(server::enemy::query::get))
        .route(
            server::resource::avatar::ROUTE,
            get(server::resource::avatar::get),
        )
        .route(
            server::resource::building_skill::ROUTE,
            get(server::resource::building_skill::get),
        )
        .route(
            server::resource::enemy::ROUTE,
            get(server::resource::enemy::get),
        )
        .route(
            server::resource::item_rarity::ROUTE,
            get(server::resource::item_rarity::get),
        )
        .route(
            server::resource::item::ROUTE,
            get(server::resource::item::get),
        )
        .route(
            server::resource::map::ROUTE,
            get(server::resource::map::get),
        )
        .route(
            server::resource::portrait::ROUTE,
            get(server::resource::portrait::get),
        )
        .route(
            server::resource::skill::ROUTE,
            get(server::resource::skill::get),
        )
        .route(
            server::resource::skin::ROUTE,
            get(server::resource::skin::get),
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
