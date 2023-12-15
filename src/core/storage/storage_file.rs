// 文件与文件差异
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileComparison{
    file_str:String,
    not_equal_line:Vec<i64>,
}

impl FileComparison{
    pub fn new(new_str:String) -> FileComparison {
        Self{
            file_str:new_str,
            not_equal_line:vec![],
        }
    }
}


pub fn compare_files(old_file: &str, new_file: &str) -> FileComparison {
    let mut file_comparison=FileComparison::new(new_file.clone().to_string());
    let long_str;
    let small_str;
    if old_file.lines().count()>new_file.lines().count() {
        long_str=old_file.clone();
        small_str=new_file.clone();
    }else {
        long_str=new_file.clone();
        small_str=old_file.clone();
    }
    for line in 0..long_str.clone().lines().count() {
         if small_str.clone().lines().nth(line)==None {
             file_comparison.not_equal_line.push(line as i64);
         }else {
             if small_str.clone().lines().nth(line)!=long_str.clone().lines().nth(line) {
                 file_comparison.not_equal_line.push(line as i64);
             }
         }
    }

    return file_comparison;
}
#[test]
pub fn test(){
    let x=compare_files("a\nbb\ncc","a\nb");
    println!("{:?}",x);
}