use std::str::FromStr;

use crate::types::medium::{
    creator::{Creator, SocialStats},
    creator_page::{ExtendedPreviewContent, PostPreview, PreviewImage},
    post::Post,
};
use http::Uri;
use std::sync::{Arc, Mutex};

use super::store::Store;
use rusqlite::{named_params, Connection, OptionalExtension, Row};

pub struct SqliteStore {
    pub conn: Arc<Mutex<Connection>>,
}

impl Store for SqliteStore {
    fn reset(&self) {
        let conn = self.conn.lock().unwrap();
        conn.execute("DROP TABLE IF EXISTS creators", ()).unwrap();
        conn.execute("DROP TABLE IF EXISTS posts", ()).unwrap();
        dbg!("Dropped tables");
        conn.execute(
            "CREATE TABLE creators(\
                id TEXT PRIMARY KEY NOT NULL,\
                name TEXT NOT NULL,\
                username TEXT NOT NULL,\
                image_id TEXT,\
                bio TEXT NOT NULL,\
                follower_count INTEGER NOT NULL\
            )",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE posts(\
                id TEXT PRIMARY KEY NOT NULL,\
                creator_id TEXT NOT NULL,\
                title TEXT NOT NULL,\
                medium_url TEXT NOT NULL,\
                created_at INTEGER NOT NULL,\
                first_published_at INTEGER NOT NULL,\
                latest_published_at INTEGER NOT NULL,\
                updated_at INTEGER NOT NULL,\
                clap_count INTEGER NOT NULL,\
                preview_image_id TEXT,\
                preview_image_alt TEXT,\
                reading_time INTEGER NOT NULL,\
                unique_slug NOT NULL,\
                subtitle TEXT NOT NULL,\
                license TEXT NOT NULL,\
                paywall BOOLEAN NOT NULL,\
                content TEXT NOT NULL\
            )",
            (),
        )
        .unwrap();
    }

    fn store_creator(&self, creator: &Creator) {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO creators\
                (id, name, username, image_id, bio, follower_count) VALUES (:id, :name, :username, :image_id, :bio, :follower_count)\
            ",
            named_params! {
                ":id": &creator.id,
                ":name": &creator.name,
                ":username": &creator.username,
                ":image_id": &creator.image_id,
                ":bio": &creator.bio,
                ":follower_count": &creator.social_stats.follower_count,
            },
        ).unwrap();
    }

    fn store_post_preview(&self, post_preview: &PostPreview) {
        todo!()
    }

    fn store_post(&self, post: &Post) {
        let conn = self.conn.lock().unwrap();
        let serialised_content = serde_json::to_string(&post.content).unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO posts\
                (\
                    id,\
                    creator_id,\
                    title,\
                    medium_url,\
                    created_at,\
                    first_published_at,\
                    latest_published_at,\
                    updated_at,\
                    clap_count,\
                    preview_image_id,\
                    preview_image_alt,\
                    reading_time,\
                    unique_slug,\
                    subtitle,\
                    license,\
                    paywall,\
                    content\
                ) VALUES (\
                    :id,\
                    :creator_id,\
                    :title,\
                    :medium_url,\
                    :created_at,\
                    :first_published_at,\
                    :latest_published_at,\
                    :updated_at,\
                    :clap_count,\
                    :preview_image_id,\
                    :preview_image_alt,\
                    :reading_time,\
                    :unique_slug,\
                    :subtitle,\
                    :license,\
                    :paywall,\
                    :content
                )",
            named_params! {
                ":id": &post.id,
                ":creator_id": &post.creator.id,
                ":title": &post.title,
                ":medium_url": &post.medium_url.to_string(),
                ":created_at": &post.created_at,
                ":first_published_at": &post.first_published_at,
                ":latest_published_at": &post.latest_published_at,
                ":updated_at": &post.updated_at,
                ":clap_count": &post.clap_count,
                ":preview_image_id": post.preview_image.as_ref().map(|x| x.id.clone()),
                ":preview_image_alt": post.preview_image.as_ref().map(|x| x.alt.clone()),
                ":reading_time": &post.reading_time,
                ":unique_slug": &post.unique_slug,
                ":subtitle" : &post.preview.subtitle,
                ":license": &post.license,
                ":paywall": &post.paywall,
                ":content": &serialised_content,
            },
        )
        .unwrap();
    }

    fn get_creators(&self) -> Vec<Creator> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM creators").unwrap();
        stmt.query_map([], |row| Ok(Creator::from_prefix("", row)))
            .unwrap()
            .map(|x| x.unwrap())
            .collect()
    }

    fn get_creator(&self, creator_id: &str) -> Creator {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT * FROM creators WHERE id = :creator_id")
            .unwrap();
        stmt.query_row(named_params! { ":creator_id": creator_id }, |row| {
            Ok(Creator::from_prefix("", row))
        })
        .unwrap()
    }

    fn get_creator_by_name(&self, username: &str) -> Creator {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT * FROM creators WHERE username = :username")
            .unwrap();
        stmt.query_row(named_params! { ":username": username }, |row| {
            Ok(Creator::from_prefix("", row))
        })
        .unwrap()
    }

    fn get_post_previews(&self, creator: &str) -> Vec<PostPreview> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT * FROM posts p inner JOIN creators c ON p.creator_id = c.id WHERE c.username = :creator_name",
            )
            .unwrap();
        stmt.query_map(
            named_params! {
                ":creator_name": &creator,
            },
            |row| {
                let serialised_url: String = row.get("medium_url").unwrap();
                let medium_url = Uri::from_str(&serialised_url).unwrap();

                let preview_image_id: Option<String> = row.get("preview_image_id").unwrap();
                let preview_image_alt: Option<String> = row.get("preview_image_alt").unwrap();
                let preview_image = match (preview_image_id) {
                    Some(id) => Some(PreviewImage {
                        id,
                        alt: preview_image_alt,
                    }),
                    _ => None,
                };

                Ok(PostPreview {
                    id: row.get("id").unwrap(),
                    medium_url: medium_url,
                    created_at: row.get("created_at").unwrap(),
                    first_published_at: row.get("first_published_at").unwrap(),
                    latest_published_at: row.get("latest_published_at").unwrap(),
                    updated_at: row.get("updated_at").unwrap(),
                    clap_count: row.get("clap_count").unwrap(),
                    preview_image: preview_image,
                    reading_time: row.get("reading_time").unwrap(),
                    unique_slug: row.get("unique_slug").unwrap(),
                    extended_preview_content: ExtendedPreviewContent {
                        subtitle: row.get("subtitle").unwrap(),
                    },
                    title: row.get("title").unwrap(),
                })
            },
        )
        .unwrap()
        .map(|x| x.unwrap())
        .collect()
    }

    fn get_post(&self, post_id: &str) -> Option<Post> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT \
                    p.id as p_id,\
                    p.creator_id as p_creator_id,\
                    p.title as p_title,\
                    p.medium_url as p_medium_url,\
                    p.created_at as p_created_at,\
                    p.first_published_at as p_first_published_at,\
                    p.latest_published_at as p_latest_published_at,\
                    p.updated_at as p_updated_at,\
                    p.clap_count as p_clap_count,\
                    p.preview_image_id as p_preview_image_id,\
                    p.preview_image_alt as p_preview_image_alt,\
                    p.reading_time as p_reading_time,\
                    p.unique_slug as p_unique_slug,\
                    p.subtitle as p_subtitle,\
                    p.license as p_license,\
                    p.paywall as p_paywall,\
                    p.content as p_content,\
                    c.id as c_id,\
                    c.name as c_name,\
                    c.username as c_username,\
                    c.image_id as c_image_id,\
                    c.bio as c_bio,\
                    c.follower_count as c_follower_count \
                 FROM posts p inner JOIN creators c ON p.creator_id = c.id WHERE p.id = :id",
            )
            .unwrap();
        stmt.query_row(
            named_params! {
                ":id": &post_id,
            },
            |row| {
                let serialised_content: String = row.get("p_content").unwrap();
                let content = serde_json::from_str(&serialised_content).unwrap();

                let serialised_url: String = row.get("p_medium_url").unwrap();
                let medium_url = Uri::from_str(&serialised_url).unwrap();

                let preview_image_id: Option<String> = row.get("p_preview_image_id").unwrap();
                let preview_image_alt: Option<String> = row.get("p_preview_image_alt").unwrap();
                let preview_image = match (preview_image_id) {
                    Some(id) => Some(PreviewImage {
                        id,
                        alt: preview_image_alt,
                    }),
                    _ => None,
                };

                Ok(Post {
                    id: row.get("p_id").unwrap(),
                    creator: Creator::from_prefix("c_", row),
                    paywall: row.get("p_paywall").unwrap(),
                    medium_url: medium_url,
                    created_at: row.get("p_created_at").unwrap(),
                    first_published_at: row.get("p_first_published_at").unwrap(),
                    latest_published_at: row.get("p_latest_published_at").unwrap(),
                    updated_at: row.get("p_updated_at").unwrap(),
                    clap_count: row.get("p_clap_count").unwrap(),
                    preview_image: preview_image,
                    reading_time: row.get("p_reading_time").unwrap(),
                    unique_slug: row.get("p_unique_slug").unwrap(),
                    preview: ExtendedPreviewContent {
                        subtitle: row.get("p_subtitle").unwrap(),
                    },
                    title: row.get("p_title").unwrap(),
                    license: row.get("p_license").unwrap(),
                    content: content,
                })
            },
        )
        .optional()
        .unwrap()
    }
}

trait FromSqlPrefix: Sized {
    fn from_prefix(prefix: &str, row: &Row) -> Self;
}

impl FromSqlPrefix for Creator {
    fn from_prefix(prefix: &str, row: &Row) -> Self {
        Creator {
            id: row.get(format!("{}id", prefix).as_str()).unwrap(),
            name: row.get(format!("{}name", prefix).as_str()).unwrap(),
            image_id: row.get(format!("{}image_id", prefix).as_str()).unwrap(),
            social_stats: SocialStats {
                follower_count: row
                    .get(format!("{}follower_count", prefix).as_str())
                    .unwrap(),
            },
            username: row.get(format!("{}username", prefix).as_str()).unwrap(),
            bio: row.get(format!("{}bio", prefix).as_str()).unwrap(),
        }
    }
}
