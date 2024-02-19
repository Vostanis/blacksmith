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

const DATA_PATH: &str = "./data";

#[tokio::main]
async fn main() { 

    let mut api = API::new();

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

    #[header("User-Agent", "example@example_domain.com")]
    #[header("API-Token", "XXXXXXXXX")]
    #[requests(3)]
    #[seconds(2)]
    api.get_vec(urls, DATA_PATH).await;

    // function macro
    // #[threads(30)]
    // // #[requests_per_second(10)]
    // #[header("User-Agent", "example@example_domain.com")]
    // #[header("API-Token", "XXXXXXXXX")]
    // download!(urls, "./some/path");
}
