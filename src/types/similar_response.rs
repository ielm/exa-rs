use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarResponse {
    #[serde(rename = "autopromptString")]
    pub autoprompt_string: Option<String>,
    pub results: Option<Vec<SimilarResult>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarResult {
    pub id: String,
    pub url: String,
    pub title: String,
    #[serde(rename = "publishedDate")]
    pub published_date: String,
    pub author: Option<String>,
    pub score: f64,
}
