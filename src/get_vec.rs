// use blacksmith_macros::collect;
use futures::StreamExt;

pub async fn get_vec(urls: Vec<&str>, save_path: &str, threads: usize) {
    let client = reqwest::Client::new();
    futures::stream::iter(urls.into_iter().map(|url| {
        let future = client
            .get(url)
            .send();
        async move {            
            match future.await {
                Ok(resp) => {
                    match resp.bytes().await {
                        Ok(bytes) => {
                            download_url_file(url, bytes, save_path).await;
                        },
                        Err(_) => eprintln!("[ERROR]   Failed to retrieve bytes: {url}"), 
                    }
                },
                Err(_) => eprintln!("[ERROR]   Failed to return response: {url}"),
            }
        }
    }))
    .buffer_unordered(threads)
    .collect::<Vec<()>>()
    .await;
}

pub async fn download_url_file(url: &str, bytes: bytes::Bytes, save_path: &str) {
    if let Some(file) = std::path::Path::new(&url).file_name() {
        let file_path = std::path::Path::new(save_path).join(file);
        let _ = tokio::fs::write(&file_path, bytes).await;
        println!("[SUCCESS] File written {file_path:?}");
    } else {
        eprintln!("[ERROR]   File name unreadable: {url}")
    }
}
