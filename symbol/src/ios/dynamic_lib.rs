use anyhow::{Result};
use object::{Object,ObjectSegment};
use symbolic_debuginfo::{Object as symbolObject};
use std::{fs};
use memmap2::Mmap;
use crate::util;
use symbolic_common::ByteView;
use goblin::{error, Object as goblinObject};


pub fn get_symbol_table(symbol_path:&str)->Result<(),anyhow::Error>{
    let file = fs::File::open(&symbol_path)?;
    let mmap = unsafe{Mmap::map(&file)?};
    let object = object::File::parse(&*mmap)?;

    //get vm_addr
    let mut segments = object.segments();
    let mut vm_addr:u64 = 0;
    while let Some(segment) = segments.next(){
        if let Some(name) = segment.name()?{
            if name == "__TEXT"{
                vm_addr = segment.address();
                break;
            }
        } 
    }
    let symbols  = object.symbol_map();
    for symbol_item in symbols.symbols(){
        if symbol_item.address()>vm_addr{
            println!("{:x} {:?}",symbol_item.address()-vm_addr,util::demangle_name(symbol_item.name()))
        }
    }
    Ok(())
}


pub fn get_symbol_table_symbolic(symbol_path:&str)->Result<(),anyhow::Error>{
    let view = ByteView::open(&symbol_path)?;
    let object = symbolObject::parse(&view)?;
    let mut symbols :Vec<_>= object.symbols().collect();
    symbols.sort_by_key(|sym|sym.address);
    for symbol_item in symbols.into_iter(){
        println!("{:x} {:?}",symbol_item.address,util::demangle_name(&symbol_item.name.unwrap().to_string()));
    }
    Ok(())
}

pub fn get_symbol_table_goblin(symbol_path:&str)->Result<(),anyhow::Error>{
    let buffer = fs::read(symbol_path)?;
    match goblinObject::parse(&buffer)? {
        goblinObject::Elf(elf) => {
           for item in elf.dynsyms.into_iter(){
                println!("{:?}",item)
           }
        },
        goblinObject::PE(pe) => {
            println!("pe: {:#?}", &pe);
        },
        goblinObject::Mach(mach) => {
            println!("mach: {:#?}", &mach);
        },
        goblinObject::Archive(archive) => {
            println!("archive: {:#?}", &archive);
        },
        goblinObject::Unknown(magic) => { println!("unknown magic: {:#x}", magic) }
    }
    Ok(())
}