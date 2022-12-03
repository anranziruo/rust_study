use symbol::ios::dynamic_lib;
use symbol::windows::sys_pdb;
use symbol::windows::app_pdb;

#[test]
fn test_ios_symbol_table(){
    let dmp_detail = dynamic_lib::get_symbol_table("../test_data/ios/lib/libc++.1.dylib");
    match dmp_detail {
        Ok(v) => {
            // println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}

#[test]
fn test_ios_symbol_table_symbolic(){
    let dmp_detail = dynamic_lib::get_symbol_table_symbolic("../test_data/ios/lib/libc++.1.dylib");
    match dmp_detail {
        Ok(v) => {
            // println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}

#[test]
fn test_ios_symbol_table_goblin(){
    let dmp_detail = dynamic_lib::get_symbol_table_goblin("../test_data/ios/lib/libc++.1.dylib");
    match dmp_detail {
        Ok(v) => {
            // println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}



#[test]
fn test_windows_sys_pdb_symbolic(){
    let dmp_detail = sys_pdb::get_sym_pdb_symbolic("../test_data/windows/sys/dcomp.pdb");
    match dmp_detail {
        Ok(v) => {
            // println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}

#[test]
fn test_windows_sys_pdb(){
    let dmp_detail = sys_pdb::get_sym_pdb("../test_data/windows/sys/dcomp.pdb");
    match dmp_detail {
        Ok(v) => {
            // println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}


#[test]
fn test_app_pdb(){
    let dmp_detail = app_pdb::get_app_pdb_ymbolic("../test_data/windows/app/foo.pdb");
    match dmp_detail {
        Ok(v) => {
            // println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}