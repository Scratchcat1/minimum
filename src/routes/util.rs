use crate::templates::TimedRender;
use askama::Template;
use axum::{
    http::header,
    http::StatusCode,
    response::{IntoResponse, Response},
};

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
