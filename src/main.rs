#![feature(proc_macro_hygiene)]
#[allow(dead_code)]
#[allow(unused_imports)]

use blacksmith::{
    get,
    runner::Runner,
};
// use blacksmith::get;
// use blacksmith::runner::*;
use blacksmith_macros::*;

#[tokio::main]
async fn main() { 
    let urls = vec![
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
    ];

    let runner = Runner::new();
    runner.get_vec(urls.clone(), "./dump", 1).await;

    // #[header2("this", "that")]
    get!(urls, "./src", 3); // needs await here 
}
