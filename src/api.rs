#[derive(Debug)]
pub struct API {
    pub client_builder: reqwest::ClientBuilder,
    pub headers: reqwest::header::HeaderMap,
    pub threads: u16,
    // pub requests_per_second: u16,
}

impl API {
    pub fn new() -> Self {
        API {
            client_builder: reqwest::ClientBuilder::new(),
            headers: reqwest::header::HeaderMap::new(),
            threads: 1,
            // requests_per_second: 100,
        }
    }

    pub async fn get_vec(
        &self, 
        urls: Vec<&str>, 
        save_path: &str
    ) {
        use futures::StreamExt;

        let client = reqwest::ClientBuilder::new()
            .default_headers(self.headers.clone())
            .build()
            .expect("Failed to build client");

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
                                Runner::download_url(url, bytes, save_path).await;
                            },
                            Err(_) => eprintln!("Failed to retrieve byte: {url}")
                        }
                    },
                    Err(_) => eprintln!("Failed to retrieve response: {url}"),
                }
            }
        }))
        .buffer_unordered(&self.threads)
        .collect::<Vec<()>>()
        .await;
    } 

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

macro_rules! download {
    ($urls:ident, $path:literal) => {
        let mut api = API::new();
        api.get_vec($urls, $path).await;
    };
}
