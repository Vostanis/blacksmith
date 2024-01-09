#![feature(proc_macro_hygiene)]
#[allow(dead_code)]
#[allow(unused_imports)]

use blacksmith::get_vec::*;
use blacksmith::get_vec2::Runner;
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
    
    // #[header("This", "that")]
    // get_vec(urls.clone(), "./src", 1).await;

    let runner = Runner::new();
    runner.get_vec(urls, "./src", 1).await;
}

// #[header("User-Agent", "kimonvostanis@gmail.com")] -- does this work??
macro_rules! get_vec {
    // declare runner and run
    ($urls:ident, $path:literal, $threads:literal) => {
        let runner = Runner::new();
        runner.get_vec($urls, $path, $threads);
    }

    // second iteration, with a proc_macro establishing a headermap
    ($urls:ident, $path:literal, $threads:literal, $headers:ident) => {
        let runner = Runner::new();
        // header_map
        runner.get_vec($urls, $path, $threads);
    }
}
