use crate::types::medium::{
    creator::Creator,
    creator_page::{PagingInfo, PostPreview},
};
use askama::Template;

#[derive(Template)]
#[template(path = "creator_page.html")]
pub struct CreatorPageTemplate {
    pub creator: Creator,
    pub post_previews: Vec<PostPreview>,
    pub paging_info: PagingInfo,
}

impl CreatorPageTemplate {
    pub fn new(creator: Creator, post_previews: Vec<PostPreview>) -> Self {
        return Self {
            creator,
            post_previews,
            paging_info: PagingInfo {
                previous: None,
                next: None,
            },
        };
    }
}
