use crate::types::medium::creator::Creator;
use crate::types::medium::post::{Paragraph, Post};
use askama::Template;
use chrono::{DateTime, Utc};
use http::Uri;

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub creator: &'a Creator,
    pub license: &'a str,
    pub clap_count: &'a u64,
    pub medium_url: &'a Uri,
    pub first_published_at: &'a u64,
    pub paragraphs: &'a [Paragraph],
}

impl<'a> PostTemplate<'a> {
    pub fn first_published_at_date(&self) -> Option<DateTime<Utc>> {
        return DateTime::from_timestamp((self.first_published_at / 1_000) as i64, 0);
    }
}

impl<'a> From<&'a Post> for PostTemplate<'a> {
    fn from(value: &'a Post) -> Self {
        return Self {
            id: &value.id,
            title: &value.title,
            creator: &value.creator,
            license: &value.license,
            clap_count: &value.clap_count,
            medium_url: &value.medium_url,
            first_published_at: &value.first_published_at,
            paragraphs: &value.content.body_model.paragraphs,
        };
    }
}
