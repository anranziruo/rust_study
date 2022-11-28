use symbolic_common::ByteView;
use symbolic_debuginfo::{elf::ElfObject, FileEntry, Function, Object, SymbolMap};
use crate::util;
use pdb::FallibleIterator;
use std::fs::File;

pub fn get_sym_pdb_symbolic(symbol_path:&str)->Result<(),anyhow::Error>{
    let view = ByteView::open(&symbol_path)?;
    let object = Object::parse(&view)?;
    let symbols = object.symbols();
    for symbol_item in symbols.into_iter(){
        println!("{:x} {:?}",symbol_item.address,util::demangle_name(&symbol_item.name.unwrap().to_string()));
    }
    Ok(())
}

pub fn get_sym_pdb(symbol_path:&str)->Result<(),anyhow::Error> {
    let file = File::open(symbol_path)?;
    let mut pdb = pdb::PDB::open(file)?;
    let symbol_table = pdb.global_symbols()?;
    let mut symbols = symbol_table.iter();
    let address_map = pdb.address_map()?;
    while let Some(symbol) = symbols.next()? {
        match symbol.parse() {
            Ok(pdb::SymbolData::Public(data)) if data.function => {
                let rva = data.offset.to_rva(&address_map).unwrap_or_default().0;
                println!("{:x} {:?}", rva, util::demangle_name(&data.name.to_string()));
            }
            _ => {}
        }
    }
    Ok(())
}

