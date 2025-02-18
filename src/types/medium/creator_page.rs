use askama::Template;
use axum::http::Uri;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserResult<T> {
    pub data: UserResultInner<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserResultInner<T> {
    #[serde(rename = "userResult")]
    pub user_result: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatorPage {
    pub id: String,
    pub name: String,
    pub username: String,
    #[serde(rename = "imageId")]
    pub image_id: String,
    #[serde(rename = "homepagePostsConnection")]
    pub post_previews: CreatorPagePostsConnection,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatorPagePostsConnection {
    pub posts: Vec<PostPreview>,
    #[serde(rename = "pagingInfo")]
    pub paging_info: PagingInfo,
}

#[derive(Deserialize, Serialize, Debug, Template)]
#[template(path = "post_preview.html")]
pub struct PostPreview {
    pub id: String,
    pub title: String,
    #[serde(rename = "mediumUrl", with = "http_serde::uri")]
    pub medium_url: Uri,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "firstPublishedAt")]
    pub first_published_at: u64,
    #[serde(rename = "latestPublishedAt")]
    pub latest_published_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
    #[serde(rename = "clapCount")]
    pub clap_count: u64,
    #[serde(rename = "previewImage")]
    pub preview_image: Option<PreviewImage>,
    #[serde(rename = "readingTime")]
    pub reading_time: f32,
    #[serde(rename = "uniqueSlug")]
    pub unique_slug: String,
    #[serde(rename = "extendedPreviewContent")]
    pub extended_preview_content: ExtendedPreviewContent,
}

impl PostPreview {
    pub fn reading_minutes(&self) -> u16 {
        return self.reading_time as u16;
    }

    pub fn created_at_date(&self) -> Option<DateTime<Utc>> {
        return DateTime::from_timestamp((self.created_at / 1_000) as i64, 0);
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PreviewImage {
    pub id: String,
    pub alt: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtendedPreviewContent {
    pub subtitle: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PagingInfo {
    pub previous: Option<FromPagingInfo>,
    pub next: Option<FromPagingInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FromPagingInfo {
    pub from: String,
    pub limit: u32,
}
