use symbolic_debuginfo::{Object,Function};
use symbolic_common::ByteView;

pub fn get_app_dsym_symbolic(symbol_path:&str)->Result<(),anyhow::Error>{
    let view = ByteView::open(&symbol_path)?;
    let object = Object::parse(&view)?;
    let sessions = object.debug_session()?;
    for session in sessions.functions(){
        let sess = session?;
        for sess_item in sess.lines.into_iter(){
            println!("subgrame:{:?}",sess_item)
        }
        for inline_item in sess.inlinees{
            reverse_function(inline_item)
        }
    }
    Ok(()) 
}

pub fn reverse_function(
    fn_item:Function,
){
    for fn_info in fn_item.lines.into_iter(){
        println!("inline:{:?}",fn_info)
    }
    for inline_item in fn_item.inlinees{
        reverse_function(inline_item)
    }
}

#[test]
fn test_app_app_dsym_symbolic(){
    let dmp_detail = get_app_dsym_symbolic("../test_data/ios/app/crash.dSYM/Contents/Resources/DWARF/crash");
    match dmp_detail {
        Ok(v) => {
            // println!("{:?}",serde_json::to_value(v).unwrap().to_string());
            },
        Err(e) => println!("occur err: {e:?}"),
    }
}