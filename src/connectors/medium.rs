use reqwest::{Client, Response};

use crate::types::medium::creator_page::{CreatorPage, UserResult};
use crate::types::medium::post::Post;
use crate::types::medium::post::PostResult;
use tokio::time::Instant;

pub struct BasicMediumConnector {
    client: Client,
}

impl BasicMediumConnector {
    pub fn new() -> Self {
        return Self {
            client: Client::new(),
        };
    }

    fn get_post_graphql() -> String {
        include_str!("graphql/get_post.graphql").replace("\n", " ")
    }
    fn get_post_previews_graphql() -> String {
        return include_str!("graphql/get_post_previews.graphql").replace("\n", " ");
    }

    pub async fn get_post(&self, post_id: &str) -> Result<Post, String> {
        let request_start = Instant::now();

        let res = self.graph_ql(format!("[{{\"operationName\":\"PostPageQuery\",\"variables\":{{\"postId\":\"{}\",\"postMeteringOptions\":{{}}}},\"query\":\"{}\"}}]", post_id, Self::get_post_graphql())).await;

        let request_description = format!("get post {}", post_id);
        BasicMediumConnector::log_request_outcome(&request_description, &res, &request_start);

        let mut result: Vec<PostResult<Post>> = res.json().await.unwrap();
        let post = result.remove(0).data.post_result;
        return Ok(post);
    }

    pub async fn get_post_previews(
        &self,
        username: &str,
        creator_page_posts_from: Option<&str>,
    ) -> Result<CreatorPage, String> {
        let request_start = Instant::now();

        let res = self.graph_ql(format!("[{{\"operationName\":\"CreatorsQuery\",\"variables\":{{\"homepagePostsFrom\":\"{}\",\"username\":\"{}\",\"creator_pagePostsLimit\":25}},\"query\":\"{}\"}}]", creator_page_posts_from.unwrap_or(""), username, Self::get_post_previews_graphql())).await;

        let request_description = format!("get post previews {}", username);
        BasicMediumConnector::log_request_outcome(&request_description, &res, &request_start);

        let mut result: Vec<UserResult<CreatorPage>> = res.json().await.unwrap();
        let post = result.remove(0).data.user_result;
        return Ok(post);
    }

    fn log_request_outcome(
        request_description: &str,
        response: &Response,
        request_start: &Instant,
    ) {
        let duration = request_start.elapsed().as_millis();
        if response.status().is_success() {
            println!(
                "{}: ({}, {}ms)",
                request_description,
                response.status(),
                duration
            );
        } else {
            eprintln!(
                "{}: ({}, {}ms)",
                request_description,
                response.status(),
                duration
            );
        }
    }

    async fn graph_ql(&self, query: String) -> Response {
        let request = self
            .client
            .post("https://medium.com/_/graphql")
            .header("Content-Type", "application/json")
            .body(query)
            .build()
            .unwrap();
        return self.client.execute(request).await.unwrap();
    }
}
