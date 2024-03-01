use serde::{Deserialize, Serialize};

use crate::types::ContentsOptions;

use crate::ExaRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentsRequest {
    pub ids: Vec<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub options: Option<ContentsOptions>,
}

impl ContentsRequest {
    pub fn new(ids: Vec<String>) -> Self {
        ContentsRequest {
            ids,
            options: None,
        }
    }

    pub fn options(mut self, options: ContentsOptions) -> Self {
        self.options = Some(options);
        self
    }
}

impl ExaRequest for ContentsRequest {
    fn as_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
