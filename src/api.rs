/////////////////////////////////////////////////////////////////////
///
/// The API Struct
/// ==============
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
    pub threads: usize,
    pub requests_per_second: u16,
}

impl API {

    ///////////////////////////////////////////////////////////
    /// Builds an api with a default config, i.e.,
    /// single-threaded, with a max capacity of 
    /// 1000 requests per second (designed to be infeasible).
    ///
    pub fn new() -> Self {
            API {
            client_builder: reqwest::ClientBuilder::new(),
            headers: reqwest::header::HeaderMap::new(),
            threads: 1,
            requests_per_second: 1000,
        }
    }

    ///////////////////////////////////////////////////////////
    /// Iterate over a vector of endpoints (&str or String),
    /// and download their contents, as with bytes, to a
    /// specified $FILE_PATH.
    ///
    /// TODO!: Trait with generic type defn needed
    ///
    pub async fn get_vec(
        &self, 
        urls: Vec<&str>, 
        save_path: &str
    ) {
        use futures::StreamExt;

        let client = reqwest::ClientBuilder::new()
            .default_headers(self.headers.clone())
            .build()
            .expect("failed to build client");

        futures::stream::iter(urls.into_iter().map(|url| {
            let future = client
                .get(url)
                .send();

            async move {
                match future.await {
                    Ok(resp) => {
                        match resp.bytes().await {
                            Ok(bytes) => {
                                println!("downloading file: {url}");
                                API::download_url(url, bytes, save_path).await;
                            },
                            Err(_) => eprintln!("failed to retrieve bytes: {url}")
                        }
                    },
                    Err(_) => eprintln!("failed to retrieve response: {url}"),
                }
            }
        }))
        .buffer_unordered(self.threads)
        .collect::<Vec<()>>()
        .await;
    } 

    ///////////////////////////////////////////////////////////
    /// Take a URL name, the bytes recieved, and a file path.
    /// Downlad the file to the a file path, using a 
    /// derived file name.
    ///
    pub async fn download_url(
        url: &str, 
        bytes: bytes::Bytes, 
        save_path: &str
    ) {
        let file = std::path::Path::new(&url)
            .file_name()
            .expect("failed to retrieve a file name");
        let file_path = std::path::Path::new(save_path).join(file);
        let _ = tokio::fs::write(&file_path, bytes).await;
    }
}

 
 
///////////////////////////////////////////////////////////////////////////////
/// TODO!: Ease of use functional macros
// macro_rules! download {
//     ($urls:ident, $path:literal) => {
//         let mut download_from_api = API::new();
//         download_from_api.get_vec($urls, $path).await;
//     };
// }
