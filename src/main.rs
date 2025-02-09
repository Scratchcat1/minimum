use axum::{routing::get, Router};
use m_rs_lib::cache::json_cache::JsonCache;
use m_rs_lib::connectors::cached_medium::CachedMediumConnector;
use m_rs_lib::connectors::graphql_medium::GraphQlMediumConnector;
use m_rs_lib::connectors::medium::MediumConnector;
use m_rs_lib::crawler;
use m_rs_lib::crawler::crawler::Crawler;
use m_rs_lib::persistence::sqlite::SqliteStore;
use m_rs_lib::persistence::store::Store;
use m_rs_lib::routes::asset::get_asset;
use m_rs_lib::routes::creator::get_creator;
use m_rs_lib::routes::homepage::get_homepage;
use m_rs_lib::routes::post::{get_post_endpoint, get_unique_slug_post};
use m_rs_lib::routes::state::AppState;
use rusqlite::Connection;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    let store = SqliteStore {
        conn: Arc::new(Mutex::new(Connection::open("./db.sqlite").unwrap())),
    };
    let post_cache = JsonCache::<String> {
        time_to_live: Duration::from_secs(3600),
        base_path: PathBuf::from_str("./cache").unwrap(),
        get_file_path: Box::new(|post_id| PathBuf::from_str(&format!("{}.json", post_id)).unwrap()),
    };
    let post_preview_cache = JsonCache::<String> {
        time_to_live: Duration::from_secs(3600),
        base_path: PathBuf::from_str("./cache").unwrap(),
        get_file_path: Box::new(|key| PathBuf::from_str(&format!("{}.json", key)).unwrap()),
    };
    let creator_cache = JsonCache::<String> {
        time_to_live: Duration::from_secs(3600),
        base_path: PathBuf::from_str("./cache").unwrap(),
        get_file_path: Box::new(|key| PathBuf::from_str(&format!("{}.json", key)).unwrap()),
    };
    let medium_connector = GraphQlMediumConnector::new();
    let cached_medium_connector = CachedMediumConnector {
        post_cache,
        post_preview_cache,
        creator_cache,
        connector: medium_connector,
    };

    let crawler = Crawler {
        medium: Box::new(cached_medium_connector),
        store: Box::new(store),
    };
    crawler.crawl_user("admiralcloudberg");

    let app = Router::new()
        .route("/", get(get_homepage))
        .route("/creator/:username", get(get_creator))
        .route("/posts/by-id/:post_id", get(get_post_endpoint))
        .route("/posts/by-slug/:unique_slug", get(get_unique_slug_post))
        .route("/assets/:asset_name", get(get_asset))
        .layer(CompressionLayer::new())
        .with_state(Arc::new(AppState { crawler }));

    println!("Starting server");
    axum::Server::bind(&"0.0.0.0:9080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
