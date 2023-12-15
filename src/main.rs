mod config;
mod test;
mod core;
mod utils;

use std::env;
use core::ages::get_version;
use crate::config::path::find_path::find_dir;
use crate::config::path::find_path::FindMode::{Top};

fn main(){
    // let mit_dir=find_dir(".mit",env::current_dir().unwrap(),Top);
    // let x=test(mit_dir.clone().unwrap(),mit_dir.unwrap().to_str().unwrap());
    // println!("{:#?}",x);
    run();
}

fn run(){
    let args=env::args();
    if let Some(arg)=args.into_iter().nth(1){
        match arg.as_str() {
            "init"=>{
                match core::ages::init() {
                    Ok(ok)=>{
                        println!("{}",ok);
                    },
                    Err(er)=>{
                        println!("{}",er);
                    }
                }
            },
            "--version"=>{
                // 使用find_dir 查看你父文件下是否有.mit文件
                match find_dir(".mit", env::current_dir().unwrap(), Top) {
                    None => {
                        println!("不在.mit管理范围的文件夹")
                    }
                    Some(mut dir) => {
                        println!("你的.mit文件在{:?}",dir.as_path());
                        dir.push(".mit");
                        dir.push(".config");
                        println!("{:?}", dir);
                        get_version(dir);
                    }
                }
                // let path=PathBuf::from(".mit/.config");
                // get_version(path);
            },
            _=>{
                println!("啥都不是")
            }
        }
    }
}

