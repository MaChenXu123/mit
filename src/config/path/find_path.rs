use std::fs;
use std::path::{Path, PathBuf};




// 用来看当前命令执行目录是否是mit子目录
// find_name用来找到指定文件名 path_buf 指定从哪个地址开始寻找 mode 指定是向上寻找还是向下寻找
pub fn find_dir(find_name:&str,path_buf: PathBuf,mode:FindMode)->Option<PathBuf> {
    match find_path_mode(find_name,path_buf,mode) {
        Some(dir)=>{
            return Some(dir);
        },
        None=>{
            None
        }
    }
}
#[derive(Debug)]
pub enum FindMode{
    Top,
    Bottom
}

fn find_path_mode(s: &str, path: PathBuf,mode:FindMode) -> Option<PathBuf>{
   match mode {
       FindMode::Top=>{
           for x in fs::read_dir(path.clone()).unwrap() {
               if x.unwrap().file_name().to_str().unwrap()==".mit" {
                   return Some(path);
               }
           }
           if path.file_name()==None {
               println!("{:?}", path.file_name());
               return None;
           }
           return crate::config::path::find_path::find_path_mode(s, PathBuf::from(path.parent().unwrap()), mode)
       },
       FindMode::Bottom=>{
           if let Err(e) = path.read_dir(){
               return None;
           }
           for x in fs::read_dir(path.clone()).unwrap() {
               if x.unwrap().file_name().to_str().unwrap()==".mit" {
                   return Some(path);
               }
           }

           for x in path.read_dir().unwrap() {
               return find_path_mode(s,x.unwrap().path(),mode);
           }
       }
   }
    None
}
