use reqwest::Client;

use crate::types::medium::creator_page::{CreatorPage, UserResult};
use crate::types::medium::post::Post;
use crate::types::medium::post::PostResult;

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
        "query PostPageQuery($postId: ID!, $postMeteringOptions: PostMeteringOptions) {
    postResult(id: $postId) {
        ... on Post {
            id
            creator {
                id
                imageId
                name
                socialStats {
                    followerCount
                }
                username
                hasSubdomain
                customDomainState {
                    live {
                        domain
                    }
                }
                bio
            }
            content(postMeteringOptions: $postMeteringOptions) {
                bodyModel {
                    paragraphs {
                        id
                        name
                        type
                        href
                        layout
                        metadata {
                            id
                            originalHeight
                            originalWidth
                            focusPercentX
                            focusPercentY
                            alt
                        }
                        text
                        hasDropCap
                        dropCapImage {
                            id
                            originalHeight
                            originalWidth
                        }
                        markups {
                            type
                            start
                            end
                            href
                            anchorType
                            userId
                            linkMetadata {
                                httpStatus
                            }
                        }
                        codeBlockMetadata {
                            mode
                            lang
                        }
                        iframe {
                            mediaResource {
                                id
                                iframeSrc
                                iframeHeight
                                iframeWidth
                                title
                            }
                        }
                        mixtapeMetadata {
                            href
                            mediaResource {
                                mediumCatalog {
                                    id
                                }
                            }
                        }
                    }
                }
            }
            inResponseToEntityType
            isLocked
            isMarkedPaywallOnly
            mediumUrl
            topics {
                slug
            }
            postResponses {
                count
            }
            createdAt
            firstPublishedAt
            latestPublishedAt
            clapCount
            title
            uniqueSlug
            readingTime
            previewContent {
                subtitle
            }
            previewImage {
                id
                alt
                focusPercentX
                focusPercentY
                originalHeight
                originalWidth
            }
            updatedAt
            license
            tags {
                id
                displayTitle
                normalizedTagSlug
            }
        }
    }
}
"
        .to_string()
        .replace("\n", " ")
    }
    fn get_post_previews_graphql() -> String {
        return "
query CreatorsQuery(
    $id: ID
    $username: ID
    $homepagePostsLimit: PaginationLimit
    $homepagePostsFrom: String = null
) {
    userResult(id: $id, username: $username) {
        ... on User {
            id
            name
            username
            imageId
            homepagePostsConnection(
                paging: { limit: $homepagePostsLimit, from: $homepagePostsFrom }
            ) {
                posts {
                    id
                    title
                    postResponses {
                        count
                    }
                    createdAt
                    firstPublishedAt
                    latestPublishedAt
                    updatedAt
                    mediumUrl
                    clapCount
                    previewImage {
                        id
                    }
                    readingTime
                    uniqueSlug
                    tags {
                        id
                        displayTitle
                        normalizedTagSlug
                    }
                }
                pagingInfo {
                    previous {
                        from
                        limit
                    }
                    next {
                        from
                        limit
                    }
                }
            }
        }
    }
}
"
        .to_string()
        .replace("\n", " ");
    }

    pub async fn get_post(&self, post_id: &str) -> Result<Post, String> {
        println!("{:?}", format!("[{{\"operationName\":\"PostPageQuery\",\"variables\":{{\"postId\":\"{}\",\"postMeteringOptions\":{{}}}},\"query\":\"{}\"}}]", post_id, Self::get_post_graphql()));
        let request = self.client.post("https://medium.com/_/graphql")
            .header("Content-Type", "application/json")
            .body(format!("[{{\"operationName\":\"PostPageQuery\",\"variables\":{{\"postId\":\"{}\",\"postMeteringOptions\":{{}}}},\"query\":\"{}\"}}]", post_id, Self::get_post_graphql()))
            .build().unwrap();
        let res = self.client.execute(request).await.unwrap();
        println!("Status: {}", res.status());
        if !res.status().is_success() {
            panic!("Request failed with HTTP {}", res.status());
        }
        // println!("body: {}" , res.text().await.unwrap());
        let mut result: Vec<PostResult<Post>> = res.json().await.unwrap();
        let post = result.remove(0).data.post_result;
        return Ok(post);
    }

    pub async fn get_post_previews(
        &self,
        username: &str,
        creator_page_posts_from: Option<&str>,
    ) -> Result<CreatorPage, String> {
        let request = self.client.post("https://medium.com/_/graphql")
            .header("Content-Type", "application/json")
            .body(format!("[{{\"operationName\":\"CreatorsQuery\",\"variables\":{{\"creator_pagePostsFrom\":\"{}\",\"username\":\"{}\",\"creator_pagePostsLimit\":25}},\"query\":\"{}\"}}]", creator_page_posts_from.unwrap_or(""), username, Self::get_post_previews_graphql()))
            .build().unwrap();
        let res = self.client.execute(request).await.unwrap();
        println!("Status: {}", res.status());
        if !res.status().is_success() {
            eprintln!("Request failed with HTTP {}", res.status());
            eprintln!("Body: {}", res.text().await.unwrap());
            panic!("!")
        }
        // println!("body: {}" , res.text().await.unwrap());
        let mut result: Vec<UserResult<CreatorPage>> = res.json().await.unwrap();
        let post = result.remove(0).data.user_result;
        return Ok(post);
    }
}
