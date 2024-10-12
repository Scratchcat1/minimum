use super::AppState;
use super::{html_ok, internal_server_error};
use crate::templates::creator_page::CreatorPageTemplate;
use axum::extract::{Path, Query, State};
use axum::{
    body::BoxBody,
    http::header,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::Instant;

pub async fn get_user_redirect(Query(params): Query<HashMap<String, String>>) -> Response<BoxBody> {
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

pub async fn get_posts(
    Path(username): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Response<BoxBody> {
    let request_start = Instant::now();
    let posts_from = params.get("from");
    println!(
        "Request: get_posts(username={}, posts_from={:?})",
        username, posts_from
    );
    let post_previews_result = state
        .medium
        .get_post_previews(&username, posts_from.map(|x| x.as_str()))
        .await;
    println!("Request in {}ms", request_start.elapsed().as_millis());
    match post_previews_result {
        Ok(post_previews) => {
            let html = CreatorPageTemplate::from(&post_previews);
            html_ok(html)
        }
        Err(e) => internal_server_error(format!("Something went wrong: {e}")),
    }
}
