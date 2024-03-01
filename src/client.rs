use reqwest::Url;
use reqwest::{Client, Error, Response};

use crate::constants;
use crate::types::{
    ContentsRequest, ContentsResponse, SearchRequest, SearchResponse, SimilarRequest,
    SimilarResponse,
};
use crate::ExaRequest;

#[derive(Clone, Debug)]
pub struct Exa {
    api_key: String,
    client: Client,
    url: Url,
}

impl Default for Exa {
    fn default() -> Self {
        Exa::new()
    }
}

impl Exa {
    pub fn new() -> Self {
        Exa {
            api_key: std::env::var("EXA_API_KEY").expect("EXA_API_KEY must be set"),
            client: Client::builder().user_agent("exa-rs").build().unwrap(),
            url: Url::parse(constants::DEFAULT_BASE_URL).unwrap(),
        }
    }

    pub fn with_api_key(api_key: String) -> Self {
        Exa {
            api_key,
            client: Client::builder().user_agent("exa-rs").build().unwrap(),
            url: Url::parse(constants::DEFAULT_BASE_URL).unwrap(),
        }
    }

    pub async fn request(&self, route: &str, req: impl ExaRequest) -> Result<Response, Error> {
        let url = self.url.join(route).unwrap();

        println!("url: {:?}", url.as_str());
        // TODO(@ielm) - handle errors
        let res = self
            .client
            .post(url.as_str())
            .header("x-api-key", &self.api_key)
            .header("User-Agent", "exa-rs")
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&req.as_json())
            .send()
            .await?;
        // println!("res: {:?}", res);

        Ok(res)
    }

    pub async fn search(&self, req: SearchRequest) -> Result<SearchResponse, Error> {
        let res = self
            .request(constants::DEFAULT_SEARCH_PATH, req)
            .await?
            .json::<SearchResponse>()
            .await?;
        Ok(res)
    }

    pub async fn find_similar(&self, req: SimilarRequest) -> Result<SimilarResponse, Error> {
        let res = self
            .request(constants::DEFAULT_FIND_SIMILAR_PATH, req)
            .await?;
        println!("res: {:?}", res);

        let res = res.json::<SimilarResponse>().await?;
        Ok(res)
    }

    pub async fn get_contents(&self, req: ContentsRequest) -> Result<ContentsResponse, Error> {
        let res = self
            .request(constants::DEFAULT_CONTENTS_PATH, req)
            .await?
            .json::<ContentsResponse>()
            .await?;
        Ok(res)
    }
}
