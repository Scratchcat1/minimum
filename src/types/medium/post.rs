use crate::types::medium::creator;
use http::Uri;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub id: String,
    pub creator: creator::Creator,
    #[serde(rename = "isMarkedPaywallOnly")]
    pub paywall: bool,
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
    pub title: String,
    pub license: String,
    pub content: Content,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Content {
    #[serde(rename = "bodyModel")]
    pub body_model: BodyModel,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BodyModel {
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Paragraph {
    #[serde(rename = "type")]
    pub p_type: String,
    pub text: String,
    pub metadata: Option<Metadata>,
    pub markups: Option<Vec<Markup>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub id: String,
    #[serde(rename = "originalHeight")]
    pub original_height: Option<u32>,
    #[serde(rename = "originalWidth")]
    pub original_width: Option<u32>,
    pub alt: Option<String>,
}

impl Metadata {
    pub fn is_video(&self) -> bool {
        return self.id.ends_with(".gif");
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Markup {
    #[serde(rename = "type")]
    pub markup_type: String,
    pub start: u32,
    pub end: u32,
    pub href: Option<String>,
    #[serde(rename = "anchorType")]
    pub anchor_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostResultInner<T> {
    #[serde(rename = "postResult")]
    pub post_result: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostResult<T> {
    pub data: PostResultInner<T>,
}
