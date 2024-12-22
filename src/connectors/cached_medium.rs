use super::medium::MediumConnector;
use crate::cache::cache::Cache;
use crate::types::medium::creator_page::CreatorPage;
use crate::types::medium::post::Post;

pub struct CachedMediumConnector<
    M: MediumConnector,
    PC: Cache<String, Post>,
    PPC: Cache<String, CreatorPage>,
> {
    pub post_cache: PC,
    pub post_preview_cache: PPC,
    pub connector: M,
}

impl<'a, M: MediumConnector, PC: Cache<String, Post>, PPC: Cache<String, CreatorPage>>
    MediumConnector for CachedMediumConnector<M, PC, PPC>
{
    fn get_post(&self, post_id: &str) -> Result<Post, String> {
        match self.post_cache.get(&post_id.to_string()) {
            Some(post) => Ok(post),
            None => {
                let post = self.connector.get_post(post_id)?;
                self.post_cache.put(&post_id.to_string(), &post);
                Ok(post)
            }
        }
    }

    fn get_post_previews(
        &self,
        username: &str,
        creator_page_posts_from: Option<&str>,
    ) -> Result<CreatorPage, String> {
        let key = format!(
            "{} - {}",
            username,
            creator_page_posts_from.unwrap_or("default")
        );
        match self.post_preview_cache.get(&key) {
            Some(post) => Ok(post),
            None => {
                let post = self
                    .connector
                    .get_post_previews(username, creator_page_posts_from)?;
                self.post_preview_cache.put(&key, &post);
                Ok(post)
            }
        }
    }
}
