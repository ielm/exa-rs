use serde::{Deserialize, Serialize};

use crate::types::{ContentsOptions, RequestOptions};
use crate::ExaRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(flatten)]
    pub options: RequestOptions,
    #[serde(rename = "type")]
    search_type: SearchType,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<ContentsOptions>,
}

impl SearchRequest {
    pub fn new(query: String) -> Self {
        SearchRequest {
            query,
            options: RequestOptions::new(),
            search_type: SearchType::new(),
            contents: None,
        }
    }

    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = search_type;
        self
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

impl ExaRequest for SearchRequest {
    fn as_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    #[default]
    #[serde(rename = "neural")]
    Neural,
    #[serde(rename = "keyword")]
    Keyword,
}

impl SearchType {
    pub fn new() -> Self {
        SearchType::default()
    }
}
