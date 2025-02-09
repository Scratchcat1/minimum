use reqwest::blocking::{Client, Response};
use tokio::task::block_in_place;

use crate::types::medium::creator::Creator;
use crate::types::medium::creator_page::{CreatorPage, UserResult};
use crate::types::medium::post::Post;
use crate::types::medium::post::PostResult;
use tokio::time::Instant;

use super::medium::MediumConnector;

pub struct GraphQlMediumConnector {
    client: Client,
}

impl GraphQlMediumConnector {
    pub fn new() -> Self {
        let client = block_in_place(|| Client::new());
        return Self { client };
    }

    fn get_post_graphql() -> String {
        include_str!("graphql/get_post.graphql").replace("\n", " ")
    }
    fn get_post_previews_graphql() -> String {
        return include_str!("graphql/get_post_previews.graphql").replace("\n", " ");
    }
    fn get_creator_graphql() -> String {
        include_str!("graphql/get_creator.graphql").replace("\n", " ")
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

    fn graph_ql(&self, query: String) -> Response {
        block_in_place(|| {
            let request = self
                .client
                .post("https://medium.com/_/graphql")
                .header("Content-Type", "application/json")
                .body(query)
                .build()
                .unwrap();
            self.client.execute(request).unwrap()
        })
    }
}

impl MediumConnector for GraphQlMediumConnector {
    fn get_post(&self, post_id: &str) -> Result<Post, String> {
        let request_start = Instant::now();

        let res = self.graph_ql(format!("[{{\"operationName\":\"PostPageQuery\",\"variables\":{{\"postId\":\"{}\",\"postMeteringOptions\":{{}}}},\"query\":\"{}\"}}]", post_id, Self::get_post_graphql()));

        let request_description = format!("get post {}", post_id);
        GraphQlMediumConnector::log_request_outcome(&request_description, &res, &request_start);

        let mut result: Vec<PostResult<Post>> = block_in_place(|| res.json().unwrap());
        let post = result.remove(0).data.post_result;
        return Ok(post);
    }

    fn get_post_previews(
        &self,
        username: &str,
        creator_page_posts_from: Option<&str>,
    ) -> Result<CreatorPage, String> {
        let request_start = Instant::now();

        let res = self.graph_ql(format!("[{{\"operationName\":\"CreatorsQuery\",\"variables\":{{\"homepagePostsFrom\":\"{}\",\"username\":\"{}\",\"creator_pagePostsLimit\":25}},\"query\":\"{}\"}}]", creator_page_posts_from.unwrap_or(""), username, Self::get_post_previews_graphql()));

        let request_description = format!("get post previews {}", username);
        GraphQlMediumConnector::log_request_outcome(&request_description, &res, &request_start);

        let mut result: Vec<UserResult<CreatorPage>> = block_in_place(|| res.json().unwrap());
        let post = result.remove(0).data.user_result;
        return Ok(post);
    }

    fn get_creator(&self, username: &str) -> Result<Creator, String> {
        let request_start = Instant::now();

        let res = self.graph_ql(format!(
            "[{{\"operationName\":\"CreatorsQuery\",\"variables\":{{\"username\":\"{}\",\"creator_pagePostsLimit\":25}},\"query\":\"{}\"}}]",
            username,
            Self::get_creator_graphql()
        ));

        let request_description = format!("get_creator {}", username);
        GraphQlMediumConnector::log_request_outcome(&request_description, &res, &request_start);

        let mut result: Vec<UserResult<Creator>> = block_in_place(|| res.json().unwrap());
        let post = result.remove(0).data.user_result;
        Ok(post)
    }
}
