use super::state::AppState;
use crate::routes::util::{html_ok, internal_server_error};
use crate::templates::post::PostTemplate;
use axum::extract::{Path, State};
use axum::{
    body::BoxBody,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use regex::Regex;
use std::sync::Arc;

pub fn get_post(post_id: &str, state: Arc<AppState>) -> Response<BoxBody> {
    println!("Request: get_post(post_id={})", post_id);
    let post_result = state.crawler.get_post(&post_id).ok_or("");
    match post_result {
        Ok(post) => {
            let html = PostTemplate::from(&post);
            html_ok(html)
        }
        Err(e) => internal_server_error(format!("Something went wrong: {e}")),
    }
}

pub async fn get_post_endpoint(
    Path(post_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response<BoxBody> {
    return get_post(&post_id, state);
}

pub async fn get_unique_slug_post(
    Path(unique_slug): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response<BoxBody> {
    println!("Request: get_unique_slug_post(unique_slug={})", unique_slug);
    let regex = Regex::new(r"([[:xdigit:]]{12}$)").unwrap();
    let captures = regex.captures(&unique_slug);
    match captures {
        Some(captures) => get_post(&captures[0], state),
        None => (StatusCode::NOT_FOUND, "Missing post id!").into_response(),
    }
}
