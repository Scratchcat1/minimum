use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Creator {
    pub id: String,
    pub name: String,
    #[serde(rename = "imageId")]
    pub image_id: String,
    #[serde(rename = "socialStats")]
    pub social_stats: SocialStats,
    pub username: String,
    pub bio: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SocialStats {
    #[serde(rename = "followerCount")]
    pub follower_count: u32,
}
