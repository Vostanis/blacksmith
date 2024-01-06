#![feature(proc_macro_hygiene)]
#[allow(dead_code)]
#[allow(unused_imports)]

use blacksmith::get_vec::*;
use blacksmith_macros::*;
use futures::StreamExt;

#[tokio::main]
async fn main() { 
    let urls = vec![
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
    ];

    // #[header("User-Agent", "example@example.com")]
    // get_vec(urls, "./src", 1).await;
    
    #[header("This", "that")]
    get_vec(urls, "./src", 1).await;
}
