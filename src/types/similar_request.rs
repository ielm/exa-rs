use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::{ContentsOptions, RequestOptions};
use crate::ExaRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarRequest {
    pub url: String,
    #[serde(flatten)]
    pub options: RequestOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<ContentsOptions>,
}

impl SimilarRequest {
    pub fn new(url: Url) -> Self {
        SimilarRequest {
            url: url.to_string(),
            options: RequestOptions::new(),
            contents: None,
        }
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn contents(mut self, contents: ContentsOptions) -> Self {
        self.contents = Some(contents);
        self
    }
}

impl ExaRequest for SimilarRequest {
    fn as_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}