use std::path::Path;

use crate::{
    connectors::medium::MediumConnector,
    persistence::store::Store,
    types::medium::{creator::Creator, creator_page::PostPreview, post::Post},
};
use reqwest::blocking::{get, Client, Response};
use tokio::task::block_in_place;

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
                let post = self.store.get_post(&post_preview.id).unwrap_or_else(|| {
                    println!("Fetching post {} for {}", post_preview.id, username);
                    let post = self.medium.get_post(&post_preview.id).unwrap();
                    self.store.store_post(&post);
                    post
                });
                self.crawl_post_media(&post);
            }

            if let Some(next) = &current_post.post_previews.paging_info.next {
                current_post = self
                    .medium
                    .get_post_previews(&username, Some(&next.from))
                    .unwrap();
            } else {
                break;
            }
        }
    }

    fn crawl_post_media(&self, post: &Post) {
        println!("Downloading media for post {}", post.id);
        if let Some(img) = &post.preview_image {
            self.save_post_image(&post.id, &img.id);
        }
        for paragraph in &post.content.body_model.paragraphs {
            if let Some(metadata) = &paragraph.metadata {
                if !metadata.id.is_empty() {
                    self.save_post_image(&post.id, &metadata.id);
                }
            }
        }
    }

    fn save_post_image(&self, post_id: &str, image_id: &str) {
        let folder = Path::new("media").join(post_id);
        let base_path = folder.join(image_id);
        let (path, url) = match image_id.ends_with(".gif") {
            true => (
                base_path.with_extension("mp4"),
                format!("https://miro.medium.com/v2/format:mp4/{}", image_id),
            ),
            false => (
                base_path.with_extension("jpg"),
                format!("https://miro.medium.com/v2/format:jpeg/{}", image_id),
            ),
        };
        if path.exists() {
            println!("Already have image for {}: {}", post_id, image_id)
        } else {
            println!("Saving image for {}: {}", post_id, image_id);
            if !folder.is_dir() {
                std::fs::create_dir(&folder).unwrap();
            }
            let mut response = block_in_place(|| get(url).unwrap());
            let mut file = std::fs::File::create(path).expect("Failed to create file");
            block_in_place(|| std::io::copy(&mut response, &mut file).unwrap());
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
