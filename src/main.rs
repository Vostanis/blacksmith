#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#[allow(dead_code)]
#[allow(unused_imports)]

use blacksmith_macros::*;
use blacksmith::{
    // get,
    runner::Runner,
};

#[tokio::main]
async fn main() { 
    let urls = vec![
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
    ];

    let runner = Runner::new();

    #[header2("this", "that")]
    #[header2("THIS", "THAT")]
    runner.get_vec(urls.clone(), "./dump", 1).await;

    // #[header2("this", "that")]
    // get!(urls, "./src", 3); // needs await here 
}
