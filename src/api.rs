use anyhow::Result;
use futures::{join, StreamExt};
use std::fmt::{Debug, Display};
use tokio::time::{sleep, Duration};

/////////////////////////////////////////////////////////////////////
///
/// API is technically an APIBuilder; modelling the elements
/// required to execute an API on, rather than the API call itself.
///
/// Naming is done for convention, and elements are designed as such
/// to make macro calls (functional or procedural) are for convention,
/// too.
///
#[derive(Debug)]
pub struct API {
    pub client_builder: reqwest::ClientBuilder,
    pub headers: reqwest::header::HeaderMap,
    pub n: usize,
    pub t: u64,
}

impl API {

    ///////////////////////////////////////////////////////////
    /// Builds a reqwest Client with a default API config, 
    /// i.e., single-threaded, with a max capacity of 
    /// 1000 requests per second.
    ///
    pub fn new() -> Self {
        API {
            client_builder: reqwest::ClientBuilder::new(),
            headers: reqwest::header::HeaderMap::new(),
            n: 1,
            t: 1,
        }
    }

    ///////////////////////////////////////////////////////////
    /// Iterate over a vector of endpoints (&str or String),
    /// and download their contents, as bytes, to a
    /// specified $FILE_PATH.
    ///
    /// TODO!
    /// ====
    ///     1. Trait with generic type defn needed 
    ///        (replace Vec<_> for Iter)
    ///     2. Pin<Client> may enable referencing within async
    ///        block; read "rust async programming book"
    ///
    pub async fn get_vec(&self, urls: Vec<&str>, dir: &str) -> Result<()>
    // where
    //     T: Debug + Display
    {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;
        while y < urls.len() {

            let timer = async { sleep(Duration::from_secs(self.t)).await; };
    
            let iter = async {
                x = count * self.n;
                y = count * self.n + self.n;
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
                                    Ok(bytes) => API::write(url, bytes, dir).await,
                                    Err(_) => eprintln!("failed to download {url:#?}"),
                                }
                            },
                            Err(_) => eprintln!("failed to GET {url:#?}"),
                        }
                    }
                }))
                .buffer_unordered(self.n)
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
        dir: &str
    ) {
        let file_name = std::path::Path::new(&url)
            .file_name()
            .expect("failed to retrieve a file name");
        let file_path = std::path::Path::new(dir).join(file_name);
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
//     #[threads(5)]
//     #[request_rate(10)]
//     download!(urls);
// }
// 

///////////////////////////////////////////////
// api!() simply spawns a mutable api with a
// standardised, default name for the other
// function macros to call
// 
//
///////////////////////////////////////////////////////////////////////////////
// TODO!: Ease of use functional macros
// macro_rules! download {
//     ($urls:ident, $path:literal) => {
//         let mut download_from_api = API::new();
//         download_from_api.get_vec($urls, $path).await;
//     };
// }
// macro_rules! api {
//     () => {
//         let mut api_in_progress = API::new();
//     }
// }

