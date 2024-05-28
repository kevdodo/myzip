use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::str;
        
use std::path::Path;


// use crate::utils::get_bit;


const  COMPRESSION_METHOD_IDX : (usize, usize) = (8, 10);

fn get_file_data_idxes(file_name_length: u32, file_size : u32) -> (usize, usize) {
    (file_name_length as usize + 30, file_name_length as usize + (30 + file_size as usize))
}


fn unzip0 (zip_name: &str) {
    println!("unzipping {}", zip_name);
    let mut f = File::open(zip_name).expect("couldn't open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("couldn't read file");
    // let reversed_buffer: Vec<u8> = buffer.iter().map(|&x| x.reverse_bits()).collect();

    // let reversed_buffer = dbg!(reversed_buffer);
    // for byte in reversed_buffer{
    //     println!("{:08b}", byte);
    // }

    // let file_contents = fs::read_to_string(zip_name).expect("why didn't u read");

    let compression_method = &buffer[COMPRESSION_METHOD_IDX.0..COMPRESSION_METHOD_IDX.1];
    let compression_method = u16::from_le_bytes(compression_method.try_into().unwrap());

    println!("compression: {}", compression_method);

    // if compression_method == 0 {
    //     println!("no compression");
    // } else {
    //     println!("compression method: {}", compression_method);
    //     let deflated = String::from(zip_name) + ".deflated";
    // }
    
    let file_size = &buffer[18..22];
    let file_size = u32::from_le_bytes(file_size.try_into().unwrap());


    println!("file size {}", file_size);

    let file_name_length = &buffer[26..28];
    let file_name_length = u16::from_le_bytes(file_name_length.try_into().unwrap());

    let extra_field_length = &buffer[28..30];

    let extra_field_length = u16::from_le_bytes(extra_field_length.try_into().unwrap());

    println!("extra field length {}", extra_field_length);


    // let extra_field = ;

    let mut file_name = buffer[30 as usize..file_name_length as usize + 30 as usize].to_vec();

    println!("extra field: ");
    let extra_field = buffer[file_name_length as usize + 30..file_name_length as usize + 30 + extra_field_length as usize].to_vec();
    let extra_field_binary: Vec<String> = extra_field.iter().map(|b| format!("{:08b}", b)).collect();

    println!("extra field in binary: {:?}", extra_field_binary);
    
    if compression_method == 8 {
        file_name.extend_from_slice(b".deflate");
    };

    // keep making directories for nested zip stuff

    // WILL NEED TO REVERSE THE BUFFER WHEN DOING UNZIP WITH COMPRESSION
    // let reversed_buffer: Vec<u8> = buffer.iter().map(|&x| x.reverse_bits()).collect();
    
    // println!("Compression method {:?}", compression_method);


    
    if let Ok(filename_with_extension_str) = std::str::from_utf8(&file_name) {


        println!("Filename with .deflate extension: {}", filename_with_extension_str);


        let path = Path::new(filename_with_extension_str);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(&parent).expect("Failed to create directories");
        }

        let (file_start, file_end) = get_file_data_idxes(file_name_length as u32 + extra_field_length as u32, file_size);
        let file_data = &buffer[file_start..file_end];

        let file_data_binary: Vec<String> = file_data.iter().map(|b| format!("{:08b}", b)).collect();

        println!("file_data_binary in binary: {:?}", file_data_binary);

        // println!("{}", file_end - file_start);
    
        let mut file = File::create(filename_with_extension_str).expect("failed to open");
        file.write_all(&file_data).expect("couldn't write bytes");
    } else {
        println!("Filename with .deflate extension contains non-UTF-8 characters.");
    }
}

// Code lengths are in the dynamic code length body
// HLIT - Once we have the lengths, we can decode HLIT codes with the same algorithm

// Like before, we can 
// HDIST 


// We can store the codes as a bool vec array, or numerical values by length 



fn main() {
    let args: Vec<String> = env::args().collect();
    let zip_name : &String = &args[1];
    // let file_name : &String = &args[2];
    println!("zip name: {}", zip_name);
    unzip0(zip_name);

    // check_same(,)


    // println!("lets goooo {}", bruh[0])
}
