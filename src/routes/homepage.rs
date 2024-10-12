use super::html_ok;
use crate::templates::homepage::HomepageTemplate;
use axum::{body::BoxBody, response::Response};

pub async fn get_homepage() -> Response<BoxBody> {
    let html = HomepageTemplate { version: "Unknown" };
    return html_ok(html);
}
