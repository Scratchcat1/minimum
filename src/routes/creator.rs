use crate::routes::util::html_ok;
use crate::templates::creator_page::CreatorPageTemplate;
use axum::extract::{Path, Query, State};
use axum::{body::BoxBody, response::Response};
use std::collections::HashMap;
use std::sync::Arc;

use super::state::AppState;

pub async fn get_creator(
    Path(username): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Response<BoxBody> {
    let posts_from = params.get("from");
    let limit: u32 = params
        .get("limit")
        .map(|x| x.parse().ok())
        .flatten()
        .unwrap_or(25);

    println!(
        "Request: get_posts(username={}, posts_from={:?}, limit={:?})",
        username, posts_from, limit
    );

    let creator = state.crawler.get_creator(&username);
    let post_previews = state.crawler.get_post_previews(&username);

    let html = CreatorPageTemplate::new(creator, post_previews);
    html_ok(html)
}
