pub mod basic;
use symbol::ios::app_dsym_symbolic;
fn main() {
    let dmp_detail = app_dsym_symbolic::get_app_dsym_symbolic("../test_data/ios/app/crash.dSYM/Contents/Resources/DWARF/crash");
    match dmp_detail {
        Ok(v) => {
            // println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}
