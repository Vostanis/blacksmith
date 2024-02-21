use anyhow::Result;
use futures::{join, StreamExt};
use tokio::time::{sleep, Duration};

use std::{
    collections::HashMap,
    fmt::Debug,
    path::Path,
};

/////////////////////////////////////////////////////////////////////
/// API is technically an APIBuilder, modelling the elements
/// required to execute an API on, rather than the API call itself.
///
/// Naming is done for convention, and elements are designed as such
/// to make macro calls (functional or procedural).
///
#[derive(Debug)]
pub struct API {
    pub client_builder: reqwest::ClientBuilder,
    pub headers: reqwest::header::HeaderMap,
    pub requests: usize,
    pub seconds: u64,
}

impl API {

    ///////////////////////////////////////////////////////////
    /// Builds a reqwest Client with a default API config, 
    /// i.e., single-threaded, with a max capacity of 
    /// 1000 requests per second.
    ///
    pub fn new(requests: usize, seconds: u64) -> Self {
        API {
            client_builder: reqwest::ClientBuilder::new(),
            headers: reqwest::header::HeaderMap::new(),
            requests: requests,
            seconds: seconds,
        }
    }

    ///////////////////////////////////////////////////////////
    /// Iterate over a vector of endpoints (&str or String),
    /// and download their contents, as bytes, to a
    /// specified $FILE_PATH.
    ///
    pub async fn get_vec(
        &self, 
        urls: Vec<&str>, 
        dir: &str, 
        rename_map: Option<HashMap<&str, &str>>
    ) -> Result<()> 
    {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;
        while y < urls.len() {

            let timer = async { sleep(Duration::from_secs(self.seconds)).await; };
    
            let iter = async {
                x = count * self.requests;
                y = count * self.requests + self.requests;
                let slice = &urls[x..y];
                let client = reqwest::ClientBuilder::new()
                    .default_headers(self.headers.clone())
                    .build()
                    .expect("failed to build client");
    
                futures::stream::iter(slice.iter().map(|url| {
        
                    let rename_map = rename_map.clone();
                    let future = client
                        .get(url.to_string())
                        .send();
    
                    async move {
                        match future.await {
                            Ok(resp) => {
                                match resp.bytes().await {
                                    Ok(bytes) => API::write(url, bytes, dir, rename_map).await,
                                    Err(_) => eprintln!("failed to download {url:#?}"),
                                }
                            },
                            Err(_) => eprintln!("failed to GET {url:#?}"),
                        }
                    }
                }))
                .buffer_unordered(self.requests)
                .collect::<Vec<()>>()
                .await;
            };

            join!(timer, iter);
            count += 1;
        }

        Ok(())
    } 

    ///////////////////////////////////////////////////////////
    /// Take a URL name, the bytes recieved, and a file path.
    /// Downlad the file to the a file path, using a 
    /// derived file name.
    ///
    pub async fn write(
        url: &str, 
        bytes: bytes::Bytes, 
        dir: &str, 
        rename_map: Option<HashMap<&str, &str>>
    ) -> () {
        
        let file_name = match rename_map {

            Some(names) => names
                .get(&url).expect("file name provided"),

            None => Path::new(&url)
                .file_name().expect("retrieved a file name")
                .to_str().expect("OsStr -> str conversion"),
        };

        let file_path = Path::new(dir).join(file_name);
        let _ = tokio::fs::write(&file_path, bytes).await;
    }
}

////////////////////////////////////////////////////////////////
// Below are functional macros used for a default setup
//
// e.g., 
//
// fn main() {
//     api!();
//
//     let urls = vec![
//         "endpoint1.com/api/some_file.json",
//         "endpoint2.com/api-1/another_file.xml",
//     ];
//
//     #[header("User-Agent", "email_example@example.com")]
//     #[requests(5)]
//     #[seconds(10)]
//     download!(urls);
// }
// 
//
// api!() simply spawns a mutable api with a
// standardised, default name for the other
// function macros to call
// 
// #[macro_export]
// macro_rules! api {
//     () => {
//         let mut BLACKSMITH_API_MACRO = API::new();
//     }
// }

// #[macro_export]
// macro_rules! download {
//     ($urls:ident, $path:literal) => {
//         BLACKSMITH_API_MACRO.get_vec($urls, $path).await;
//     };
// }
