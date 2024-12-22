use crate::types::medium::{creator_page::CreatorPage, post::Post};

pub trait MediumConnector {
    fn get_post(&self, post_id: &str) -> Result<Post, String>;
    fn get_post_previews(
        &self,
        username: &str,
        creator_page_posts_from: Option<&str>,
    ) -> Result<CreatorPage, String>;
}
