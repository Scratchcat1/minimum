use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct SocialStats {
    #[serde(rename = "followerCount")]
    pub follower_count: u32,
}