use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{PathBuf};

//输入mao写入文件
pub fn write_file_from_map(map:BTreeMap<&str,&str>,path: String){
    let mut file = match File::create(path) {
        Ok(file) => {
            file
        },
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };
    for (k,v) in map {
        if k=="" {
            let s=v.to_string()+"\n";
            file.write_all(s.as_bytes()).expect("写入错误");
        }else {
            let s=format!("{} = {}\n",k,v);
            file.write_all(s.as_bytes()).expect("写入错误");
        }
        file.flush().expect("刷新文件缓冲区失败");
    }
}

// 输入文件路径 解析返回map
pub fn read_file_to_map(path:PathBuf)->Option<BTreeMap<String,String>>{
    // let mut map: HashMap<String, String> = HashMap::new();
    let mut map:BTreeMap<String,String>=BTreeMap::new();
    let mut f =File::open(path).expect("TODO: panic message");
    let buf=&mut String::new();
    f.read_to_string(buf).expect("缓冲区错误");
    let lines:Vec<&str>=buf.lines().collect();
    for line in lines {
        let a=string_segmentation(line);
        match a{
            Some(x)=>{
                map.insert(x.0,x.1);
            }
            None => {
            }
        }
    }
    return Some(map)
}

#[test]
fn test(){
}

//字符串分割
fn string_segmentation(str: &str) ->Option<(String, String)>{
    if str.chars().next().unwrap()=='[' {
        return None;
    }
    let mut s1:String = String::new();
    let mut flag=false;
    let mut s2:String = String::new();
    for c in str.chars() {
        if !flag {
            if c=='=' {
                flag=true
            }else {
                if c!=' ' {
                    s1 = s1 + c.to_string().as_str();
                }
            }
        }else {
            if c!=' ' && c!='"'{
                s2=s2+c.to_string().as_str();
            }
        }
    }
    return Some((s1,s2))
}
