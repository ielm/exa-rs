use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOptions {
    #[serde(rename = "numResults", skip_serializing_if = "Option::is_none")]
    num_results: Option<i32>,
    #[serde(rename = "includeDomains", skip_serializing_if = "Option::is_none")]
    include_domains: Option<Vec<String>>,
    #[serde(rename = "excludeDomains", skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<Vec<String>>,
    #[serde(
        rename = "startCrawlDate",
        skip_serializing_if = "Option::is_none",
        with = "chrono::serde::ts_seconds_option"
    )]
    start_crawl_date: Option<DateTime<Utc>>,
    #[serde(
        rename = "endCrawlDate",
        skip_serializing_if = "Option::is_none",
        with = "chrono::serde::ts_seconds_option"
    )]
    end_crawl_date: Option<DateTime<Utc>>,
    #[serde(rename = "startPublishedDate", skip_serializing_if = "Option::is_none")]
    start_published_date: Option<String>,
    #[serde(rename = "endPublishedDate", skip_serializing_if = "Option::is_none")]
    end_published_date: Option<String>,
    #[serde(
        rename = "excludeSourceDomain",
        skip_serializing_if = "Option::is_none"
    )]
    exclude_source_domain: Option<bool>,
    #[serde(rename = "useAutoprompt")]
    use_autoprompt: bool,
}

impl Default for RequestOptions {
    fn default() -> Self {
        RequestOptions {
            num_results: Some(10),
            include_domains: None,
            exclude_domains: None,
            start_crawl_date: None,
            end_crawl_date: None,
            start_published_date: None,
            end_published_date: None,
            exclude_source_domain: None,
            use_autoprompt: constants::DEFAULT_AUTOPROMPT,
        }
    }
}

impl RequestOptions {
    pub fn new() -> Self {
        RequestOptions::default()
    }

    pub fn builder() -> RequestOptionsBuilder {
        RequestOptionsBuilder::new()
    }

    pub fn num_results(mut self, num_results: i32) -> Self {
        self.num_results = Some(num_results);
        self
    }

    pub fn include_domains(mut self, include_domains: Vec<String>) -> Self {
        self.include_domains = Some(include_domains);
        self
    }

    pub fn exclude_domains(mut self, exclude_domains: Vec<String>) -> Self {
        self.exclude_domains = Some(exclude_domains);
        self
    }

    pub fn start_crawl_date(mut self, start_crawl_date: DateTime<Utc>) -> Self {
        self.start_crawl_date = Some(start_crawl_date);
        self
    }

    pub fn end_crawl_date(mut self, end_crawl_date: DateTime<Utc>) -> Self {
        self.end_crawl_date = Some(end_crawl_date);
        self
    }

    pub fn start_published_date(mut self, start_published_date: String) -> Self {
        self.start_published_date = Some(start_published_date);
        self
    }

    pub fn end_published_date(mut self, end_published_date: String) -> Self {
        self.end_published_date = Some(end_published_date);
        self
    }

    pub fn exclude_source_domain(mut self, exclude_source_domain: bool) -> Self {
        self.exclude_source_domain = Some(exclude_source_domain);
        self
    }

    pub fn use_autoprompt(mut self, use_autoprompt: bool) -> Self {
        self.use_autoprompt = use_autoprompt;
        self
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RequestOptionsBuilder {
    options: RequestOptions,
}

impl RequestOptionsBuilder {
    pub fn new() -> Self {
        RequestOptionsBuilder {
            options: RequestOptions::default(),
        }
    }

    pub fn num_results(mut self, num_results: i32) -> Self {
        self.options = self.options.num_results(num_results);
        self
    }

    pub fn include_domains(mut self, include_domains: Vec<String>) -> Self {
        self.options = self.options.include_domains(include_domains);
        self
    }

    pub fn exclude_domains(mut self, exclude_domains: Vec<String>) -> Self {
        self.options = self.options.exclude_domains(exclude_domains);
        self
    }

    pub fn start_crawl_date(mut self, start_crawl_date: DateTime<Utc>) -> Self {
        self.options = self.options.start_crawl_date(start_crawl_date);
        self
    }

    pub fn end_crawl_date(mut self, end_crawl_date: DateTime<Utc>) -> Self {
        self.options = self.options.end_crawl_date(end_crawl_date);
        self
    }

    pub fn start_published_date(mut self, start_published_date: String) -> Self {
        self.options = self.options.start_published_date(start_published_date);
        self
    }

    pub fn end_published_date(mut self, end_published_date: String) -> Self {
        self.options = self.options.end_published_date(end_published_date);
        self
    }

    pub fn exclude_source_domain(mut self, exclude_source_domain: bool) -> Self {
        self.options = self.options.exclude_source_domain(exclude_source_domain);
        self
    }

    pub fn use_autoprompt(mut self, use_autoprompt: bool) -> Self {
        self.options = self.options.use_autoprompt(use_autoprompt);
        self
    }

    pub fn build(self) -> RequestOptions {
        self.options
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ContentsOptions {
    text: TextOptions,
    highlights: HighlightsOptions,
    #[serde(rename = "useAutoprompt")]
    use_autoprompt: bool,
}

impl ContentsOptions {
    pub fn new() -> Self {
        ContentsOptions::default()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ContentsOptionsBuilder {
    text: TextOptions,
    highlights: HighlightsOptions,
    use_autoprompt: bool,
}

impl ContentsOptionsBuilder {
    pub fn new() -> Self {
        ContentsOptionsBuilder::default()
    }

    pub fn text(mut self, text: TextOptions) -> Self {
        self.text = text;
        self
    }

    pub fn highlights(mut self, highlights: HighlightsOptions) -> Self {
        self.highlights = highlights;
        self
    }

    pub fn use_autoprompt(mut self, use_autoprompt: bool) -> Self {
        self.use_autoprompt = use_autoprompt;
        self
    }

    pub fn build(self) -> ContentsOptions {
        ContentsOptions {
            text: self.text,
            highlights: self.highlights,
            use_autoprompt: self.use_autoprompt,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextOptions {
    #[serde(rename = "maxCharacters")]
    max_characters: i32,
    #[serde(rename = "includeHtmlTags")]
    include_html_tags: bool,
}

impl TextOptions {
    pub fn new() -> Self {
        TextOptions::default()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextOptionsBuilder {
    max_characters: i32,
    include_html_tags: bool,
}

impl TextOptionsBuilder {
    pub fn new() -> Self {
        TextOptionsBuilder::default()
    }

    pub fn max_characters(mut self, max_characters: i32) -> Self {
        self.max_characters = max_characters;
        self
    }

    pub fn include_html_tags(mut self, include_html_tags: bool) -> Self {
        self.include_html_tags = include_html_tags;
        self
    }

    pub fn build(self) -> TextOptions {
        TextOptions {
            max_characters: self.max_characters,
            include_html_tags: self.include_html_tags,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HighlightsOptions {
    #[serde(rename = "numSentences")]
    num_sentences: i32,
    #[serde(rename = "highlightsPerUrl")]
    highlights_per_url: i32,
    query: String,
}

impl HighlightsOptions {
    pub fn new() -> Self {
        HighlightsOptions::default()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HighlightsOptionsBuilder {
    num_sentences: i32,
    highlights_per_url: i32,
    query: String,
}

impl HighlightsOptionsBuilder {
    pub fn new() -> Self {
        HighlightsOptionsBuilder::default()
    }

    pub fn num_sentences(mut self, num_sentences: i32) -> Self {
        self.num_sentences = num_sentences;
        self
    }

    pub fn highlights_per_url(mut self, highlights_per_url: i32) -> Self {
        self.highlights_per_url = highlights_per_url;
        self
    }

    pub fn query(mut self, query: String) -> Self {
        self.query = query;
        self
    }

    pub fn build(self) -> HighlightsOptions {
        HighlightsOptions {
            num_sentences: self.num_sentences,
            highlights_per_url: self.highlights_per_url,
            query: self.query,
        }
    }
}
