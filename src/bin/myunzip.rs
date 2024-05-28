use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::str;
        
use std::path::Path;

mod inflate;
mod utils;
mod dynamic;
use inflate::*;
use ::utils::*;



const  COMPRESSION_METHOD_IDX : (usize, usize) = (8, 10);

fn get_file_data_idxes(file_name_length: u32, file_size : u32) -> (usize, usize) {
    (file_name_length as usize + 30, file_name_length as usize + (30 + file_size as usize))
}


fn unzip (zip_name: &str) {
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


    let file_size = &buffer[18..22];
    let file_size = u32::from_le_bytes(file_size.try_into().unwrap());


    println!("file size {}", file_size);

    let file_name_length = &buffer[26..28];
    let file_name_length = u16::from_le_bytes(file_name_length.try_into().unwrap());

    let extra_field_length = &buffer[28..30];

    let extra_field_length = u16::from_le_bytes(extra_field_length.try_into().unwrap());

    let file_name = buffer[30 as usize..file_name_length as usize + 30 as usize].to_vec();

    // let extra_field = buffer[file_name_length as usize + 30..file_name_length as usize + 30 + extra_field_length as usize].to_vec();
    // let extra_field_binary: Vec<String> = extra_field.iter().map(|b| format!("{:08b}", b)).collect();

    
    if compression_method == 8 {
        if let Ok(file_path) = std::str::from_utf8(&file_name) {
            println!("Filename: {}", file_path);
    
            let path = Path::new(file_path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(&parent).expect("Failed to create directories");
            }
    
            let (file_start, file_end) = get_file_data_idxes(file_name_length as u32 + extra_field_length as u32, file_size);

            let deflate_stream = &buffer[file_start..file_end];
            let deflate_stream: Vec<u8> = deflate_stream.iter().map(|&x| x.reverse_bits()).collect();


            let mut bfinal = false;
            let mut idx = 0; 
        
            let mut file_data = Vec::new();
        
            let mut cnt = 0;
            while !bfinal as bool {
                bfinal = get_num_reverse(&get_n_bits_reverse(&deflate_stream, idx, 1)) != 0;
                idx += 1;
                let btype = get_num_reverse(&get_n_bits_reverse(&deflate_stream, idx, 2));
                idx += 2;
            
                if btype == 2{
                    println!("compressed with dynamic codes");
                    inflate_dynamic(&deflate_stream, &mut idx, &mut file_data);
                } else if btype == 1 {
                    println!("compressed with fixed codes");

                    inflate_fixed(&deflate_stream, &mut idx, &mut file_data);
        
                } else {
                    panic!("what is btype");
                }
                cnt += 1;
                println!("num blocks: {}", cnt);
            }
        
            let mut file = File::create(file_path).expect("failed to open");
            file.write_all(&file_data).expect("couldn't write bytes");
    
            // println!("{}", file_end - file_start);
        
            // let mut file = File::create(file_path).expect("failed to open");
            // file.write_all(&file_data).expect("couldn't write bytes");
        } else {
            println!("Filename with .deflate extension contains non-UTF-8 characters.");
        }
    }
    else if compression_method == 0{

        if let Ok(file_path) = std::str::from_utf8(&file_name) {
            println!("Filename with .deflate extension: {}", file_path);
    
    
            let path = Path::new(file_path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(&parent).expect("Failed to create directories");
            }
    
            let (file_start, file_end) = get_file_data_idxes(file_name_length as u32 + extra_field_length as u32, file_size);
            let file_data = &buffer[file_start..file_end];
    
        
            let mut file = File::create(file_path).expect("failed to open");
            file.write_all(&file_data).expect("couldn't write bytes");
        }
    }
    else{
        panic!("invalid compression method");           
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
    unzip(zip_name);

}
