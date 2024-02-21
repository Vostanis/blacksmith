#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#[allow(dead_code)]
#[allow(unused_imports)]

use blacksmith::api::API;
use std::collections::HashMap;
use reqwest::header::HeaderValue;

#[tokio::main]
async fn main() { 
  
    // config
    let mut api = API::new();
    api.requests = 3;
    api.seconds = 1;
    api.headers.insert(
        "User-Agent", HeaderValue::from_str("example@example_domain.com").expect("HEADER SHOULD HAVE HEADER'D")
    );

    // list of endpoints
    let urls = vec![
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://ww.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://ww.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://ww.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://ww.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://ww.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
        String::from("https://ww.sec.gov/files/company_tickers.json"),
        String::from("https://www.sec.gov/files/company_tickers.json"),
    ];

    // map of file renaming (only 1 needed in this spam instance)
    let rename_map = HashMap::from([
        (
            String::from("https://www.sec.gov/files/company_tickers.json"), 
            String::from("beepbop.txt")
        ),
    ]);

    // make the get requests (with async clients)
    let _ = api.get_vec(urls, "./data", Some(rename_map)).await;
}
