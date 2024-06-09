use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::env;

use utils::huffman::compress;

use miniz_oxide::inflate::decompress_to_vec;
use utils::huffman::compress_threads;


fn main(){
    let args: Vec<String> = env::args().collect();
    let file_name : &String = &args[1];
    // let args = dbg!(&args);
    
    let mut f = File::open(file_name).expect("couldn't open file");
    let mut buffer = Vec::new();//"yooooooooooooo".as_bytes(); //
    f.read_to_end(&mut buffer).expect("couldn't read file");
    
    let compressed = compress_threads(buffer);
    // let compressed = compress_to_vec(&buffer, 1);
    println!("compressed length {}", compressed.len());

    let mut f = File::create("compressed_out").expect("couldn't open file");
    f.write(&compressed).expect("couldn't write");
    
    let mut f = File::open(file_name).expect("couldn't open file");
    let mut copy = Vec::new();
    f.read_to_end(&mut copy).expect("couldn't read file");
 
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
    // decompressed.push(12);
    assert_eq!(copy, decompressed);
    println!("Valid compression of {}", file_name);
 
    // decompress_to_vec_with_limit(input, max_size)
    // println!("{}", file_name)
    // myzip(file_name, zip_name);
}