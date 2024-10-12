pub mod asset;
pub mod homepage;
pub mod post;
pub mod user;
use crate::connectors::medium::BasicMediumConnector;
use crate::templates::TimedRender;
use askama::Template;
use axum::{
    http::header,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

pub struct AppState {
    pub medium: BasicMediumConnector,
}

pub fn html_ok<T: Template>(html: T) -> Response {
    return (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html.timed_render().unwrap(),
    )
        .into_response();
}

pub fn internal_server_error(error: String) -> Response {
    eprintln!("{}", error);
    return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response();
}
