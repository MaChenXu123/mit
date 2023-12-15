use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use std::io::{Read, Write};
// 压缩字符串
pub fn compress_string(input: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(input.as_bytes()).unwrap();
    encoder.finish().unwrap()
}
// 解压字符串
pub fn decompress_string(input: &[u8]) -> String {
    let mut decoder = ZlibDecoder::new(input);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap();
    decompressed
}

#[test]
fn test(){
    let original_string = "Hello, this is a test string to be compressed!";

    // 压缩字符串
    let compressed_data = compress_string(original_string);
    println!("Compressed data: {:?}", compressed_data);

    // 解压字符串
    let decompressed_string = decompress_string(&compressed_data);
    println!("Decompressed string: {}", decompressed_string);
}