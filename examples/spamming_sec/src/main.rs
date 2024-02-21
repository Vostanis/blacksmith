#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#[allow(dead_code)]
#[allow(unused_imports)]

use blacksmith::api::API;
use blacksmith_macros::{
    header,
    requests,
    seconds
};
use std::collections::HashMap;
use reqwest::header::HeaderValue;

const DATA_PATH: &str = "./data";

#[tokio::main]
async fn main() { 

    let mut api = API::new(1, 1);
    api.requests = 3;
    api.seconds = 1;
    api.headers.insert(
        "User-Agent", HeaderValue::from_str("example@example_domain.com").expect("HEADER SHOULD HAVE HEADER'D")
    );

    let urls = vec![
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
    ];

    let rename_map = HashMap::from([
        ("https://www.sec.gov/files/company_tickers.json", "beepbbop.txt"),
    ]);

    // #[header("User-Agent", "example@example_domain.com")]
    // #[header("API-Token", "XXXXXXXXX")]
    // #[requests(3)]
    // #[seconds(2)]
    let _ = api.get_vec(urls, "./data", Some(rename_map)).await;
}
