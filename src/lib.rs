pub mod get_vec;
pub mod get_vec2;


pub mod fs {

    // read json file to type T
    pub async fn read_json_file<T: serde::de::DeserializeOwned>(
        file_path: &str,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        use std::io::Read;
        let mut file = std::fs::File::open(file_path)?;
        let mut file_str = String::new();
        file.read_to_string(&mut file_str)?;
        let json: T = serde_json::from_str(&file_str)?;
        Ok(json)
    }

    // unzip file exttracts to target directory
    pub async fn unzip(zip_file_path: &str, target_dir: &str) {
        match std::fs::File::open(zip_file_path) {
            Ok(file) => {
                let buf_reader = std::io::BufReader::new(file);
                match zip_extract::extract(buf_reader, std::path::Path::new(target_dir), false) {
                    Ok(_) => println!("Extracting file: {zip_file_path}"),
                    Err(_) => println!(
                        "[ERROR]   Could not extract file {} to target directory {}",
                        zip_file_path, target_dir
                    ),
                }
            }
            Err(_) => println!("[ERROR]   Unable to open file {zip_file_path}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blacksmith_macros::collect;

    #[collect]
    fn did_this_collect() {
        ">>>>>>>>>> THIS HAS COLLECTED <<<<<<<<<<<"
    }

    #[tokio::test]
    async fn random_response() {
        todo!(); 
    }
}
