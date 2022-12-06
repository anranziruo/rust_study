use symbolic_debuginfo::{Object};
use symbolic_common::ByteView;


pub fn get_app_pdb_ymbolic(symbol_path:&str)->Result<(),anyhow::Error>{
    let view = ByteView::open(&symbol_path)?;
    let object = Object::parse(&view)?;
    let sessions = object.debug_session()?;
    for session in sessions.functions(){
        let sess = session?;
        for fun in sess.inlinees{
            println!("{:?}",fun)
        }
    }
    Ok(()) 
}