use axum::{routing::get, Router};
use m_rs_lib::cache::json_cache::JsonCache;
use m_rs_lib::connectors::cached_medium::CachedMediumConnector;
use m_rs_lib::connectors::graphql_medium::GraphQlMediumConnector;
use m_rs_lib::routes::asset::get_asset;
use m_rs_lib::routes::homepage::get_homepage;
use m_rs_lib::routes::post::{get_post_endpoint, get_unique_slug_post};
use m_rs_lib::routes::state::AppState;
use m_rs_lib::routes::user::{get_posts, get_user_redirect};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    let post_cache = JsonCache::<String> {
        base_path: PathBuf::from_str("./cache").unwrap(),
        get_file_path: Box::new(|post_id| PathBuf::from_str(&format!("{}.json", post_id)).unwrap()),
    };
    let post_preview_cache = JsonCache::<String> {
        base_path: PathBuf::from_str("./cache").unwrap(),
        get_file_path: Box::new(|key| PathBuf::from_str(&format!("{}.json", key)).unwrap()),
    };
    let medium_connector = GraphQlMediumConnector::new();
    let cached_medium_connector = CachedMediumConnector {
        post_cache,
        post_preview_cache,
        connector: medium_connector,
    };
    let app = Router::new()
        .route("/", get(get_homepage))
        .route("/user-redirect", get(get_user_redirect))
        .route("/user/:username/posts", get(get_posts))
        .route("/posts/by-id/:post_id", get(get_post_endpoint))
        .route("/posts/by-slug/:unique_slug", get(get_unique_slug_post))
        .route("/assets/:asset_name", get(get_asset))
        .layer(CompressionLayer::new())
        .with_state(Arc::new(AppState {
            medium: Box::new(cached_medium_connector),
        }));

    println!("Starting server");
    axum::Server::bind(&"0.0.0.0:9080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
