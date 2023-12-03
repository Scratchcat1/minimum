use askama::Template;
use axum::extract::{Path, Query, State};
use axum::{
    body::BoxBody,
    http::header,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use m_rs_lib::connectors::medium::BasicMediumConnector;
use m_rs_lib::templates::creator_page::CreatorPageTemplate;
use m_rs_lib::templates::homepage::HomepageTemplate;
use m_rs_lib::templates::post::PostTemplate;
use m_rs_lib::templates::TimedRender;
use regex::Regex;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::Instant;
use tower_http::compression::CompressionLayer;

struct AppState {
    medium: BasicMediumConnector,
}

async fn get_asset(Path(asset_name): Path<String>) -> Response<BoxBody> {
    match asset_name.as_str() {
        "main.css" => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
            include_str!("../assets/main.css"),
        )
            .into_response(),
        _ => (StatusCode::NOT_FOUND, "Something went wrong").into_response(),
    }
}

#[tokio::main]
async fn main() {
    let medium_connector = BasicMediumConnector::new();
    let app = Router::new()
        .route("/", get(get_homepage))
        .route("/user-redirect", get(get_user_redirect))
        .route("/user/:username/posts", get(get_posts))
        .route("/posts/:post_id", get(get_post_endpoint))
        .route("/assets/:asset_name", get(get_asset))
        .route("/:unique_slug", get(get_unique_slug_post))
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

async fn get_homepage() -> Response<BoxBody> {
    let html = HomepageTemplate { version: "Unknown" }
        .timed_render()
        .unwrap();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    )
        .into_response()
}

async fn get_user_redirect(Query(params): Query<HashMap<String, String>>) -> Response<BoxBody> {
    let username = match params.get("username") {
        Some(user) => user,
        None => return (StatusCode::BAD_REQUEST, "Missing username").into_response(),
    };
    (
        StatusCode::TEMPORARY_REDIRECT,
        [(header::LOCATION, format!("/user/{}/posts", username))],
    )
        .into_response()
}

async fn get_post(post_id: &str, state: Arc<AppState>) -> Response<BoxBody> {
    let request_start = Instant::now();
    let post_result = state.medium.get_post(&post_id).await;
    println!("Request in {}ms", request_start.elapsed().as_millis());
    match post_result {
        Ok(post) => {
            let html = PostTemplate::from(&post).timed_render().unwrap();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                html,
            )
                .into_response()
        }
        Err(e) => {
            println!("Something went wrong: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
    }
}

async fn get_post_endpoint(
    Path(post_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response<BoxBody> {
    return get_post(&post_id, state).await;
}

async fn get_unique_slug_post(
    Path(unique_slug): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response<BoxBody> {
    let regex = Regex::new(r"([[:xdigit:]]{11}$)").unwrap();
    let captures = regex.captures(&unique_slug);
    match captures {
        Some(captures) => get_post(&captures[0], state).await,
        None => (StatusCode::NOT_FOUND, "Missing post id!").into_response(),
    }
}

async fn get_posts(
    Path(username): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Response<BoxBody> {
    let request_start = Instant::now();
    let posts_from = params.get("from");
    let post_previews_result = state
        .medium
        .get_post_previews(&username, posts_from.map(|x| x.as_str()))
        .await;
    println!("Request in {}ms", request_start.elapsed().as_millis());
    match post_previews_result {
        Ok(post_previews) => {
            let html = CreatorPageTemplate::from(&post_previews)
                .timed_render()
                .unwrap();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                html,
            )
                .into_response()
        }
        Err(e) => {
            println!("Something went wrong: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
    }
}
