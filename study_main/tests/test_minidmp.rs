use minidmp::read_mini_dmp;
use common::download_binary_file;


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


