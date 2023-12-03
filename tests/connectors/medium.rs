#[cfg(test)]
mod tests {

    use m_rs_lib::types::medium::creator_page::{CreatorPage, UserResult};
    #[test]
    fn deserialize_creator_page_response() {
        let json: &str = include_str!("../resources/creator_page_response.json");
        serde_json::from_str::<Vec<UserResult<CreatorPage>>>(json).unwrap();
    }
}
