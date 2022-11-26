use symbol::ios::dynamic_lib;

#[test]
fn test_ios_symbol_table(){
    let dmp_detail = dynamic_lib::get_symbol_table("../test_data/symbols/lib/libc++.1.dylib");
    match dmp_detail {
        Ok(v) => {
            println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}