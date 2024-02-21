use anyhow::Result;
use bytes::Bytes;
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
    /// i.e., single-threaded with a max capacity of 
    /// 1000 requests per second.
    ///
    pub fn new(requests: usize, seconds: u64) -> Self {
        API {
            client_builder: reqwest::ClientBuilder::new(),
            headers: reqwest::header::HeaderMap::new(),
            requests: 1,
            seconds: 1,
        }
    }

    ///////////////////////////////////////////////////////////
    /// Iterate over a vector of endpoints (&str or String),
    /// and download their contents, as bytes, to a
    /// specified $FILE_PATH.
    ///
    pub async fn get_vec(
        &self, 
        urls: Vec<String>, 
        dir: &str, 
        name_map: Option<HashMap<String, String>>
    ) -> Result<()> {
      
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
        
                    let name_map = name_map.clone();
                    let future = client
                        .get(url)
                        .send();
    
                    async move {
                        match future.await {
                            Ok(resp) => {
                                match resp.bytes().await {
                                    Ok(bytes) => API::write(&url, bytes, dir, name_map).await,
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
    /// Take a $URL, the $BYTES recieved, and a $FILE_PATH.
    /// Download the file to the file path using a 
    /// derived file name or an optional $HASHMAP_OF_NAMES: 
    ///         
    ///     HashMap<url, file_name_to_be_given>
    ///
    pub async fn write(
        url: &String, 
        bytes: Bytes, 
        dir: &str, 
        name_map: Option<HashMap<String, String>>
    ) -> () {

        let file_name = match name_map {

            // stringify &str -> String
            Some(names) => {
                let name_ref = names.get(url).expect("file name provided");
                String::from(name_ref)
            },

            // stringify &OsStr -> &str -> String
            None => {
                let name_ref = Path::new(url)
                    .file_name()
                    .expect("retrieved a file name")
                    .to_str()
                    .expect("&str of OsStr");
                String::from(name_ref)
            },
        };
        
        let file_path = Path::new(dir).join(file_name);
        let _ = tokio::fs::write(&file_path, bytes).await;
    }
}