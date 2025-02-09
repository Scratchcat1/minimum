use crate::types::medium::{creator::Creator, creator_page::PostPreview, post::Post};

pub trait Store {
    fn reset(&self);
    fn store_creator(&self, creator: &Creator);
    fn store_post_preview(&self, post_preview: &PostPreview);
    fn store_post(&self, post: &Post);
    fn get_creators(&self) -> Vec<Creator>;
    fn get_creator(&self, creator_id: &str) -> Creator;
    fn get_creator_by_name(&self, username: &str) -> Creator;
    fn get_post_previews(&self, creator: &str) -> Vec<PostPreview>;
    fn get_post(&self, post_id: &str) -> Option<Post>;
}
