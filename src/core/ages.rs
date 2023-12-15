use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use crate::config;

pub fn init() ->Result<String,String>{
    let res=match fs::create_dir(".mit"){
        Err(_)=>{
            Err("初始化失败你的.mit文件夹已经存在".to_string())
        },
        Ok(_)=>{
            fs::File::create("./.mit/.config").expect("创建文件错误");
            let mut map=BTreeMap::new();
            map.insert("","[map]");
            map.insert("version","0.0.1");
            map.insert("b","b");
            map.insert("c","c");
            config::config_str::write_file_from_map(map, "./.mit/.config".to_string());
            Ok("初始化mit成功".to_string())
        }
    };
    res
}
pub fn get_version(fs_path:PathBuf) {
    let map=config::config_str::read_file_to_map(fs_path).unwrap();
    println!("你当前的版本是{}",map.get("version").expect("没有找到版本信息"));
}