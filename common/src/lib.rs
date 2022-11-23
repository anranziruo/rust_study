use std::path::Path;
use std::io::{Cursor, Read};
use anyhow::{self};
use base64ct::{Base64, Encoding};
use sha1::{Sha1,Digest};

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

pub fn base64_encode(data:Vec<u8>) ->String {
    return Base64::encode_string(&data);
}

pub fn base64_decode(encoded:String) ->Vec<u8>{
    return Base64::decode_vec(&encoded).unwrap()
}

pub fn sha1WithString(sha1_str:String) ->String{
    let mut sha1er = Sha1::new();
    sha1er.update(sha1_str.as_bytes());
    return format!("{:x}",sha1er.finalize())
}


