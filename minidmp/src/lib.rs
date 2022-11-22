use minidump::*;
use minidump_processor::{Symbolizer,symbols};
use anyhow::{self};
use std::path::PathBuf;
use serde::{Deserialize,Serialize};
use memmap2::*;
use std::io::Write;
use std::collections::HashMap;


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct DumpDetail {
    modules:Vec<ModuleItem>,
    unloaded_modules:Vec<ModuleItem>,
    threads:Vec<ThreadItem>,
    crashing_thread:ThreadItem,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ModuleItem{
    base_addr:Option<String>,
    end_addr:Option<String>,
    code_id:Option<String>,
    file_name:Option<String>,
    debug_file:Option<String>,
    debug_id:Option<String>,
    version:Option<String>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ThreadItem{
    frame_count:Option<u64>,
    frames:Vec<FrameItem>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct FrameItem {
    module:Option<String>,
    module_offset:Option<String>,
    offset:Option<String>,
    missing_symbols:Option<bool>,
    #[serde(default ="default_hash_map")]
    registers:HashMap<String,String>,
}

fn default_hash_map()->HashMap<String,String>{
    let h:HashMap<String,String> = HashMap::new();
    return h;
}



//读取dmp文件信息
pub async fn read_mini_dmp(data: Vec<u8>)-> Result<DumpDetail,anyhow::Error>{
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
     
    //进行反序列化
    let json_str = String::from_utf8(json_output)?;
    let mut dmp_detail:DumpDetail = serde_json::from_str(&json_str)?;

    //对齐火狐的堆栈
    let mut index = 0;
    for frame in dmp_detail.crashing_thread.frames.iter_mut(){
        if index>0{
            let frame_module_offset = frame.module_offset.clone().unwrap().as_str().replace("0x", "");
            let frame_module_offset_num = i64::from_str_radix(&frame_module_offset,16)?;
            frame.module_offset = Some(format!("{:#018x}",frame_module_offset_num+1));

            let frame_offset = frame.offset.clone().unwrap().as_str().replace("0x", "");
            let frame_offset_num = i64::from_str_radix(&frame_offset,16)?;
            frame.offset = Some(format!("{:#018x}",frame_offset_num+1));
        }
        index = index+1;
    }

    
    for thread in dmp_detail.threads.iter_mut(){
        let mut index = 0;
        for frame in thread.frames.iter_mut(){
            if index>0{
                let frame_module_offset = frame.module_offset.clone().unwrap().as_str().replace("0x", "");
                let frame_module_offset_num = i64::from_str_radix(&frame_module_offset,16)?;
                frame.module_offset = Some(format!("{:#018x}",frame_module_offset_num+1));
    
                let frame_offset = frame.offset.clone().unwrap().as_str().replace("0x", "");
                let frame_offset_num = i64::from_str_radix(&frame_offset,16)?;
                frame.offset = Some(format!("{:#018x}",frame_offset_num+1));
            }
        }
        index = index+1;
    }

    Ok(dmp_detail)
}

