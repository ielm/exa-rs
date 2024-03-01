use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "autopromptString")]
    pub autoprompt_string: Option<String>,
    pub results: Vec<SearchResult>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub url: String,
    pub title: String,
    #[serde(rename = "publishedDate")]
    pub published_date: String,
    pub author: Option<String>,
    pub score: f64,
}

