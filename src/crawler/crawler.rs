use crate::{
    connectors::medium::MediumConnector,
    persistence::store::Store,
    types::medium::{creator::Creator, creator_page::PostPreview, post::Post},
};

pub struct Crawler {
    pub medium: Box<dyn MediumConnector + Sync + Send>,
    pub store: Box<dyn Store + Sync + Send>,
}

impl Crawler {
    pub fn crawl_user(&self, username: &str) {
        let user = self.medium.get_creator(&username).unwrap();
        self.store.store_creator(&user);

        let mut current_post = self.medium.get_post_previews(&username, None).unwrap();

        loop {
            for post_preview in &current_post.post_previews.posts {
                let has_post = self.store.get_post(&post_preview.id).is_some();
                if !has_post {
                    println!("Fetching post {} for {}", post_preview.id, username);
                    let post = self.medium.get_post(&post_preview.id).unwrap();
                    self.store.store_post(&post);
                } else {
                    println!("Already have post {} for {}", post_preview.id, username);
                }
            }

            if let Some(next) = &current_post.post_previews.paging_info.next {
                current_post = self
                    .medium
                    .get_post_previews(&username, Some(&next.from.to_string()))
                    .unwrap();
            } else {
                break;
            }
        }
    }

    pub fn get_creator(&self, username: &str) -> Creator {
        self.store.get_creator_by_name(username)
    }

    pub fn get_post_previews(&self, username: &str) -> Vec<PostPreview> {
        self.store.get_post_previews(username)
    }

    pub fn get_post(&self, post_id: &str) -> Option<Post> {
        self.store.get_post(post_id)
    }
}
