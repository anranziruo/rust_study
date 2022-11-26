use minidmp::read_mini_dmp;
use common::*;
use std::{fs::File, io::Read,env};


#[tokio::test]
async fn test_read_mini_dmp(){
    let data = download_binary_file(&"https://github.com/rust-minidump/rust-minidump/raw/main/testdata/test.dmp").await;
    match data {
           Ok(v) => {
              let dmp_detail = read_mini_dmp(v).await;
              match dmp_detail {
                Ok(v) => {
                    println!("{:?}",serde_json::to_value(v).unwrap().to_string());
                },
                Err(e) => println!("occur err: {e:?}"),
              }
           },
           Err(e) => println!("occur err: {e:?}"),
    }
}

#[tokio::test]
async fn test_read_mini_dmp_local(){
    println!("{:?}",env::current_dir());
    let mut file = File::open("../test_data/dmp/test.dmp").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents);
    
    let dmp_detail = read_mini_dmp(contents).await;
        match dmp_detail {
            Ok(v) => {
                //  println!("{:?}",serde_json::to_value(v).unwrap().to_string());
                },
            Err(e) => println!("occur err: {e:?}"),
       }
}


#[test]
fn test_base64(){
    let encode = String::from("hello");
    let encode_str = base64_encode(encode.as_bytes().to_vec());
    println!("{:?}",encode_str);
    let decode_str = base64_decode(encode_str);
    let decode = String::from_utf8(decode_str).unwrap();
    println!("{:?}",decode);
}

#[test]
fn test_sha1(){
    let sha1_str = sha1WithString(String::from("123456"));
    println!("{:?}",sha1_str);
}


