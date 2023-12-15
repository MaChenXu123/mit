extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Deserializer, Serialize};
use std::{env, fs};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use crate::config::path::find_path::find_dir;
use crate::config::path::find_path::FindMode::Top;
use crate::core::storage::storage_file::FileComparison;
#[derive(Debug, Serialize, Deserialize)]
pub struct DirEntity{
    dir_name:String,
    child_dir_entity:Vec<Box<DirEntity>>,
    child_file_entity:Vec<Box<FileEntity>>
}
#[derive(Debug, Serialize, Deserialize)]
struct FileEntity{
    file_name:String,
    file_comparison:Option<FileComparison>,
}

#[test]
fn test_aa(){
    let mit_dir=find_dir(".mit",env::current_dir().unwrap(),Top);
    let x=get_dir_entity(mit_dir.clone().unwrap(),mit_dir.unwrap().to_str().unwrap());
    let json_str = serde_json::to_string(&x).unwrap();
    // let mut hasher = Sha256::new();
    // hasher.update(json_str);
    // let result = hasher.finalize();
    // let hash_string = format!("{:x}", result);
    println!("{:?}",x);

}

pub fn get_dir_entity(path:PathBuf,static_path:&str)->DirEntity{
  let dir_name=format_path(".",path.clone().to_str().unwrap(),static_path.clone().len());
    let mut dir_entity =DirEntity{
        dir_name: dir_name.parse().unwrap(),
        child_dir_entity: Vec::new(),
        child_file_entity: Vec::new(),
    };
    let fs=fs::read_dir(path).unwrap();
    for x in fs {
        match x {
            Ok(dir)=>{
                if dir.file_type().unwrap().is_dir() {
                    if dir.file_name()=="target" || dir.file_name()==".git"  {
                        continue
                    }
                    let x=get_dir_entity(dir.path().to_path_buf(),static_path);
                    dir_entity.child_dir_entity.push(Box::new(x));
                }else {
                    let file_entity=FileEntity{ file_name:dir.file_name().to_os_string().to_str().unwrap().to_string(), file_comparison: None };
                    dir_entity.child_file_entity.push(Box::new(file_entity))
                }
            },
            Err(err)=>{
                println!("{}", err);
            }
        }
    }
    return dir_entity;
}
fn format_path(add_str:&str, temp_str:&str, len: usize) ->String{
    let s=&temp_str[len..];
    format!("{}{}",add_str,s)
}
