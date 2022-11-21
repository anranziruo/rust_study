use std::path::Path;
use std::io::{Cursor, Read};
use anyhow::{self};

pub fn get_file_name(dir_name:String) -> String{
    let new_dir = dir_name.replace("\\", "/");
    let path = Path::new(&new_dir);
    let file = path.file_name();
    return file.unwrap_or_default().to_string_lossy().to_string();
}

pub async fn download_binary_file(url :&str) -> Result<Vec<u8>,anyhow::Error>{
    let response = reqwest::get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);
    let mut data = Vec::new();
    content.read_to_end(&mut data).expect("invaid data");
    Ok(data)
}