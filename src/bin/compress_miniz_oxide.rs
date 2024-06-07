use miniz_oxide::deflate::compress_to_vec;
// use miniz_oxide::inflate::decompress_to_vec_with_limit;
use std::fs;

fn roundtrip(data: &[u8]) {
    // Compress the input
    // println!("data {}", data/);
    // dbg!(data);
    let _compressed = compress_to_vec(data, 1);
    // Decompress the compressed input and limit max output size to avoid going out of memory on large/malformed input.
    // let decompressed = decompress_to_vec_with_limit(compressed.as_slice(), 60000).expect("Failed to decompress!");
    // Check roundtrip succeeded
    // assert_eq!(data, decompressed);
}

fn main() {
    let data = fs::read("test.txt").expect("failed to read zodie");
    roundtrip(&data);
}