// extern crate macros;
// use macros::header;

pub mod get_vec;
pub mod gen {        

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

    // fn append_json_file(file_path: &str, collected_fn: FnStr) {
    //     use std::fs::OpenOptions;
    //     use std::io::{Seek, SeekFrom, Write};
    //
    //     let mut file = OpenOptions::new()
    //         .create(true)
    //         .write(true)
    //         .open(file_path)
    //         .expect("Unable to open file");
    //
    //     file.seek(SeekFrom::End(0)).unwrap();
    //     let json = serde_json::to_string(&collected_fn).expect("Failed to stringify fn");
    //
    //     file.write_all(json.as_bytes()).expect("Failed to write bytes to file");
    // }

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

// pub mod blah {
//     pub fn random() {
//         println!("{}", module_path!());
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn random_response() {
        todo!(); 
    }
}
