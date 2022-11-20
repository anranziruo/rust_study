use minidump::*;
use minidump_processor::{Symbolizer,symbols};
use anyhow::{self};
use std::path::PathBuf;
use serde_json::Value;
use memmap2::*;
use std::io::Write;

//读取dmp文件信息
pub async fn read_mini_dmp(data: &mut [u8])-> Result<(),anyhow::Error>{
    //写入到mmap
    let mut mmap = MmapMut::map_anon(data.len())?;
    (&mut mmap[..]).write(&data)?;
    let mmap = mmap.make_read_only()?;

    let dump = Minidump::read(mmap).map_err(|error| error)?;

    //设置空路径
    let path = PathBuf::new();
    let symbol_paths = vec![path];
    // let options = ProcessorOptions::default();

    let provider = Symbolizer::new(symbols::simple_symbol_supplier(symbol_paths));

    let state = minidump_processor::process_minidump(&dump, &provider)
    .await
    .map_err(|error| error)?;
    
    let mut json_output = Vec::new();
    state.print_json(&mut json_output, false).map_err(|error| error)?;
    let json: Value = serde_json::from_slice(&json_output).map_err(|error| error)?;
    println!("{:?}",json.to_string());
    Ok(())
}

