use anyhow::{Result};
use object::{Object,ObjectSection,ObjectSegment};
use std::{fs};
use memmap2::Mmap;
use crate::util::demangle_name;

pub fn get_symbol_table(symbol_path:&str)->Result<(),anyhow::Error>{
    let file = fs::File::open(&symbol_path)?;
    let mmap = unsafe{Mmap::map(&file)?};
    let object = object::File::parse(&*mmap)?;

    //get vm_addr
    let mut segments = object.segments();
    let mut vm_addr:u64 = 0;
    while let Some(segment) = segments.next(){
        if let Some(name) = segment.name()?{
            if name == "_TEXT"{
                vm_addr = segment.address();
                break;
            }
        } 
    }

    let symbols  = object.symbol_map();
    for symbol_item in symbols.symbols(){
        println!("{:?}",demangle_name(symbol_item.name()))
    }
    Ok(())
}