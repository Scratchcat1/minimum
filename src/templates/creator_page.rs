use crate::types::medium::creator_page::{CreatorPage, CreatorPagePostsConnection};
use askama::Template;

#[derive(Template)]
#[template(path = "creator_page.html")]
pub struct CreatorPageTemplate<'a> {
    pub name: &'a str,
    pub image_id: &'a str,
    pub username: &'a str,
    pub post_previews: &'a CreatorPagePostsConnection,
}

impl<'a> From<&'a CreatorPage> for CreatorPageTemplate<'a> {
    fn from(value: &'a CreatorPage) -> Self {
        return Self {
            name: &value.name,
            image_id: &value.image_id,
            username: &value.username,
            post_previews: &value.post_previews,
        };
    }
}
