#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#[allow(dead_code)]
#[allow(unused_imports)]

use blacksmith_macros::header;
use blacksmith::runner::Runner;

#[tokio::main]
async fn main() { 
    let urls = vec![
        "https://www.sec.gov/files/company_tickers.json",
        "https://ww.sec.gov/files/company_tickers.json",
        "https://www.sec.gov/files/company_tickers.json",
    ];
    let save_path = "./dump";
    let threads = 1;

    let mut runner = Runner::new();

    #[header("User-Agent", "example@example_domain.com")]
    #[header("API-Token", "XXXXXXXXX")]
    runner.get_vec(urls, save_path, threads).await;
}
