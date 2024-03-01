use exa_rs::types::{contents_request, RequestOptions, SearchRequest};
use exa_rs::Exa;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let exa_client = Exa::new();

    let search_request = SearchRequest::new("covid".to_string());

    let res = exa_client
        .search(search_request)
        .await
        .expect("Failed to search");

    println!("search response: {:?}", res);

    let ids = res
        .results
        .iter()
        .map(|result| result.id.clone())
        .collect::<Vec<String>>();

    println!("\n\nids: {:?}", ids);

    let contents_request = contents_request::ContentsRequest::new(ids);

    println!("contents request: {:?}", contents_request);
    let res = exa_client
        .get_contents(contents_request)
        .await
        .expect("Failed to get contents");

    println!("contents response: {:?}", res);

    let similar_url =
        url::Url::parse("https://slatestarcodex.com/2014/07/30/meditations-on-moloch/").unwrap();

    let opts = RequestOptions::builder().num_results(10).build();
    let similar_request =
        exa_rs::types::similar_request::SimilarRequest::new(similar_url).options(opts);

    println!("\n\nsimilar request: {:?}", similar_request);

    let res = exa_client
        .find_similar(similar_request)
        .await
        .expect("Failed to find similar");

    println!("similar response: {:?}", res);
}
