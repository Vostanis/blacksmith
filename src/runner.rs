#[derive(Debug)]
pub struct Runner {
    pub client_builder: reqwest::ClientBuilder,
    pub headers: reqwest::header::HeaderMap,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            client_builder: reqwest::ClientBuilder::new(),
            headers: reqwest::header::HeaderMap::new(),
        }
    }

    pub async fn get_vec(
        &self, 
        urls: Vec<&str>, 
        save_path: &str, 
        threads:usize
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
        .buffer_unordered(threads)
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

//// TO DO
////    get!(), where runner is instantiated within the call
////    and proc_macros used to customise.
////    
//// EXAMPLE
////    #[header(...)]
////    #[threads(n)]
////    get!(urls, save_path);
////

// #[macro_export]
// macro_rules! get {
//     // declare runner and run
//     ($urls:ident, $path:literal, $threads:literal) => {
//         let runner = Runner::new();
//         runner.get_vec($urls, $path, $threads);
//     };

//     // // second iteration, with a proc_macro establishing a headermap
//     // ($urls:ident, $path:literal, $threads:literal, $headers:ident) => {
//     //     let runner = Runner::new();
//     //     // header_map
//     //     runner.get_vec($urls, $path, $threads);
//     // }
// }
