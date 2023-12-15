// use std::fs::File;
// use std::io::Write;
//
// pub fn file_write(fs: &mut File, str:&str){
//     match fs.write_all(str.as_bytes()) {
//         Ok(_) => println!("写入成功"),
//         Err(error) => {
//             println!("失败: {}", error);
//             return;
//         }
//     }
// }