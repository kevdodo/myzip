// mod lz77;

use crate::*;

use std::fs::{File};
use std::io::{Read, Write};
use std::cmp::Ordering;
// use std::{env, fs::{self}};

// use lz77::*;

pub fn reverse_huffman(num: u8) -> Vec<bool>{
    match num.cmp(&144){
        Ordering::Equal => {
            // panic!();
            return vec![true, true, false, false, true, false, false, false, false]
        },
        Ordering::Less => {
            let new_num = num + 0b00110000;
            // panic!();
            return get_n_bits_reverse(&[new_num], 0, 8);
        },
        Ordering::Greater => {
            let val: u16 = num as u16 - 144 + 400;
            let big_bytes = (val >> 8) as u8;
            let little_bytes = val as u8;
            let bruh = get_n_bits_reverse(&[big_bytes, little_bytes], 16-9, 9);
            // panic!();
            return bruh;
        }
    }
}


pub fn get_huffman(file_name: &String){
    let mut f = File::open(file_name).expect("couldn't open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("couldn't read file");    
    // let str_buffer = String::from_utf8(buffer);//fs::read_to_string(file_name).expect("not a valid file thing");
    
    
    // let str_buffer: String = buffer.iter().map(|&c| c as char).collect();
    // let str_buffer = dbg!(str_buffer);
    // let str_buffer = lz77_compression(&str_buffer);
    // buffer = dbg!(str_buffer.as_bytes().to_vec());

    // match str_buffer {

    //     Ok()
        
    // }
    // if let Ok(str_buffer) = String::from_utf8(buffer.clone()) {
    //     let b = lz77_compression(&str_buffer);
    //     buffer = b.as_bytes().to_vec();
    // }
    // let buffer = [95, 95, 95];
    let mut all_bits: Vec<bool> = Vec::new();

    // BFinal
    all_bits.push(true);

    // BTYPE
    all_bits.push(true);
    all_bits.push(false);

    for val in &buffer{
        let mut bool_arr = reverse_huffman(*val);
        all_bits.append(&mut bool_arr);
    }

    let mut eob = vec![false, false, false, false, false, false, false, false];
    all_bits.append(&mut eob);




    // let additional_zeros = 0; //if all_bits.len() % 8 != 0 {
    // //     8 - all_bits.len() % 8
    // // } else {
    // //     0
    // // };

    // for _ in 0..additional_zeros{
    //     all_bits.push(false);
    // }

    let mut out = Vec::new();
    for chunk in all_bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit {
                byte |= 1 << i;
            }
        }
        out.push(byte);
    }

    let output_file_name = format!("{}.deflate", file_name);

    let mut file = File::create(output_file_name).expect("Failed to open");
    file.write_all(&out).expect("Couldn't write bytes");
}

// fn get_huffman_bool(buffer: &[u8]){
//     // let mut f = File::open(file_name).expect("couldn't open file");
//     // let mut buffer = Vec::new();
//     // f.read_to_end(&mut buffer).expect("couldn't read file");    

//     let mut all_bits: Vec<bool> = Vec::new();

//     // BFinal
//     all_bits.push(true);

//     // BTYPE
//     all_bits.push(true);
//     all_bits.push(false);

//     for val in buffer{
//         let mut bool_arr = reverse_huffman(*val);
//         all_bits.append(&mut bool_arr);
//     }

//     let mut eob = vec![false, false, false, false, false, false, false, false];
//     all_bits.append(&mut eob);


//     let mut out = Vec::new();
//     for chunk in all_bits.chunks(8) {
//         let mut byte = 0u8;
//         for (i, &bit) in chunk.iter().enumerate() {
//             if bit {
//                 byte |= 1 << i;
//             }
//         }
//         out.push(byte);
//     }
// }
