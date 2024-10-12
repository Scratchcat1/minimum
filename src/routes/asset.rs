use axum::extract::Path;
use axum::{
    body::BoxBody,
    http::header,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

pub async fn get_asset(Path(asset_name): Path<String>) -> Response<BoxBody> {
    match asset_name.as_str() {
        "main.css" => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
            include_str!("../../assets/main.css"),
        )
            .into_response(),
        _ => (StatusCode::NOT_FOUND, "Something went wrong").into_response(),
    }
}
