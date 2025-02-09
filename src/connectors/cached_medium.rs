use super::medium::MediumConnector;
use crate::cache::cache::Cache;
use crate::types::medium::creator::Creator;
use crate::types::medium::creator_page::CreatorPage;
use crate::types::medium::post::Post;

pub struct CachedMediumConnector<
    M: MediumConnector,
    PC: Cache<String, Post>,
    PPC: Cache<String, CreatorPage>,
    CC: Cache<String, Creator>,
> {
    pub post_cache: PC,
    pub post_preview_cache: PPC,
    pub creator_cache: CC,
    pub connector: M,
}

impl<
        M: MediumConnector,
        PC: Cache<String, Post>,
        PPC: Cache<String, CreatorPage>,
        CC: Cache<String, Creator>,
    > MediumConnector for CachedMediumConnector<M, PC, PPC, CC>
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

    fn get_creator(&self, username: &str) -> Result<Creator, String> {
        match self.creator_cache.get(&username.to_string()) {
            Some(creator) => Ok(creator),
            None => {
                let creator = self.connector.get_creator(username)?;
                self.creator_cache.put(&username.to_string(), &creator);
                Ok(creator)
            }
        }
    }
}
