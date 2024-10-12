use axum::{routing::get, Router};
use m_rs_lib::connectors::medium::BasicMediumConnector;
use m_rs_lib::routes::asset::get_asset;
use m_rs_lib::routes::homepage::get_homepage;
use m_rs_lib::routes::post::{get_post_endpoint, get_unique_slug_post};
use m_rs_lib::routes::user::{get_posts, get_user_redirect};
use m_rs_lib::routes::AppState;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    let medium_connector = BasicMediumConnector::new();
    let app = Router::new()
        .route("/", get(get_homepage))
        .route("/user-redirect", get(get_user_redirect))
        .route("/user/:username/posts", get(get_posts))
        .route("/posts/by-id/:post_id", get(get_post_endpoint))
        .route("/posts/by-slug/:unique_slug", get(get_unique_slug_post))
        .route("/assets/:asset_name", get(get_asset))
        .layer(CompressionLayer::new())
        .with_state(Arc::new(AppState {
            medium: medium_connector,
        }));

    println!("Starting server");
    axum::Server::bind(&"0.0.0.0:9080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
