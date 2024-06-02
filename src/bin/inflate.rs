use std::collections::btree_set::Difference;
use std::vec;
use utils::*;
use std::env;

use std::fs::File;
use std::io::Read;
use std::io::Write;
// mod dynamic;
// pub use crate::dynamic::{get_code_length_code_matrix, get_codes_lit_dist, convert_code_lengths_matrix, decode_dynamic};
// pub use dynamic::{get_code_length_code_matrix, get_codes_lit_dist, convert_code_lengths_matrix, decode_dynamic};

pub fn inflate_dynamic(block_header: &[u8], idx: &mut usize, file_data: &mut Vec<u8>) {
    let hlit = get_num_reverse(&get_n_bits_reverse(&block_header, *idx, 5));
    *idx += 5;
    let hdist = get_num_reverse(&get_n_bits_reverse(&block_header, *idx, 5));
    *idx += 5;
    
    // dbg!(get_n_bits_reverse(&block_header, idx, 4));
    
    let hclen = get_num_reverse(&get_n_bits_reverse(&block_header, *idx, 4));
    *idx += 4;
    
    let code_lengths = utils::dynamic::get_code_length_code_matrix(&block_header, idx, hclen);

    // println!("code lengths : {}", code_lengths.len());
    let alphabet : [u16; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    
    let mat = utils::dynamic::convert_code_lengths_matrix(&code_lengths, alphabet.to_vec());
    
    // let row_start = 0;
    // let row_end = 10;
    // let col_start = 0;
    // let col_end = 10;
    
    // for row in row_start..mat.len() {
    //     for col in col_start..row_end {
    //         print!("{} ", mat[row][col]);
    //     }
    //     println!();
    // }
    
    // The codes: 
    
    // 0 -> 00
    // 2 -> 10
    // 18 -> 11
    // 01 -> 01
    
    let lit_codes = utils::dynamic::get_codes_lit_dist(&mat, &block_header, idx, hlit as usize + 257);
    
    // // println!("lit_codes matrix:");
    // // for row in row_start..lit_codes.len() {
    // //     for col in col_start..col_end {
    // //         print!("{} ", lit_codes[row][col]);
    // //     }
    // //     println!();
    // // }
    // // dbg!(lit_codes);
    
    let dist_codes = utils::dynamic::get_codes_lit_dist(&mat, &block_header, idx, hdist as usize + 1); 
    // println!("dist codes matrix: ");
    // for row in row_start..dist_codes.len() {
    //     for col in col_start..col_end {
    //         print!("{} ", dist_codes[row][col]);
    //     }
    //     println!();
    // }        // dbg!(dist_codes);
    
    utils::dynamic::decode_dynamic(lit_codes, dist_codes, &block_header, idx, file_data);
    // Vec::new()
}


// TODO: wtf is this???
pub fn inflate_fixed(buffer: &[u8], idx: &mut usize, file_data: &mut Vec<u8>){
    decode_deflate(buffer, idx, file_data);
}


fn main(){
    env::set_var("RUST_BACKTRACE", "1");


    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide a single argument: the input file name");
    }

    let input_file_name = &args[1];
    let output_file_name = input_file_name.strip_suffix(".deflate").expect("Input file name must end with .deflate");

    let mut f = File::open(input_file_name).expect("couldn't open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("couldn't read file");
    let block_header: Vec<u8> = buffer.iter().map(|&x| x.reverse_bits()).collect();
    
    // let block_header = dbg!(block_header);

    
    let mut bfinal = false;
    let mut idx = 0; 

    let mut file_data = Vec::new();

    let mut cnt = 0;
    while !bfinal as bool {
        bfinal = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 1)) != 0;
        idx += 1;
        let btype = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 2));
        idx += 2;
    
        if btype == 2{
            // println!("dynamic stuff");
            inflate_dynamic(&block_header, &mut idx, &mut file_data);

        } else if btype == 1 {
            inflate_fixed(&block_header, &mut idx, &mut file_data);

        } else {
            panic!("what is btype");
        }
        cnt += 1;
        println!("num blocks: {}", cnt);
    }

    let mut file = File::create(output_file_name).expect("failed to open");
    file.write_all(&file_data).expect("couldn't write bytes");
    
    
}
