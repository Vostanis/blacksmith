use anyhow::Result;
use futures::{join, StreamExt};
use tokio::time::{sleep, Duration};

use std::{
    convert::AsRef,
    collections::HashMap,
    ffi::OsStr,
    fmt::{Debug, Display},
    hash::Hash,
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

type Input = dyn AsRef<OsStr>;
type Endpoints = HashMap<Input, Input>;

#[macro_export]
macro_rules! urls {
    ()
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
    pub async fn get_vec<T>(&self, urls: Vec<T>, dir: &str, file_names: Option<HashMap<T, T>>) -> Result<()> 
    where
        T: Debug + Display + AsRef<OsStr>,
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
    
                    let future = client
                        .get(url.to_string())
                        .send();
    
                    async move {
                        match future.await {
                            Ok(resp) => {
                                match resp.bytes().await {
                                    Ok(bytes) => API::write(url, bytes, dir, file_names).await,
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
    pub async fn write<T>(url: T, bytes: bytes::Bytes, dir: &str, file_names: Option<HashMap<T, T>>) -> ()
    where
        T: Debug + Display + AsRef<OsStr> + Eq + PartialEq + Hash
    {
        if let Some(names) = file_names {
            let file_name = names.get(&url)
                .expect("File name found")
                .to_string();
            let file_path = std::path::Path::new(dir).join(file_name);
            let _ = tokio::fs::write(&file_path, bytes).await;
        } else {
            let file_name = std::path::Path::new(&url)
                .file_name()
                .expect("failed to retrieve a file name");
            let file_path = std::path::Path::new(dir).join(file_name);
            let _ = tokio::fs::write(&file_path, bytes).await;
        }
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
