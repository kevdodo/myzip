use std::collections::HashMap;
use std::collections::HashSet;

use fxhash::FxHashMap;
use fxhash::FxHashSet;

// use std::io::Read;
pub const LENGTH_STARTING : [u16; 6] = [3, 11, 19, 35, 67, 131];

pub const DISTANCE_CODES: [u16; 14] = [0, 5, 9, 17, 33, 65, 129, 257, 513, 1025, 2049, 4097, 8193, 16385];

// mod huffman_old;
use core::cmp::Ordering;
use crate::*;


fn reverse_huffman(num: u8) -> Vec<bool>{
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

fn _get_temp_matches_buffer(buffer_idx: usize, buffer: &Vec<u8>) -> FxHashMap<(u8, u8, u8), usize>{
    let mut temp_matches = FxHashMap::default();

    if buffer_idx as i32 - 3 >= 0 {
        temp_matches.insert((buffer[buffer_idx-3], buffer[buffer_idx-2], buffer[buffer_idx-1]), buffer_idx-3);
    }
    if buffer_idx as i32 - 2 >= 0 {
        temp_matches.insert((buffer[buffer_idx-2], buffer[buffer_idx-1], buffer[buffer_idx-2]), buffer_idx - 2);
    }
    if buffer_idx > 0{
        temp_matches.insert((buffer[buffer_idx-1], buffer[buffer_idx-1], buffer[buffer_idx-1]), buffer_idx-1);
    }
    temp_matches
}

fn find_match_buffer(buffer: &Vec<u8>, buffer_idx: &usize, true_matches: &mut FxHashMap<(u8, u8, u8), FxHashSet<usize>>) -> Option<(usize, usize)>{

    /*
    
    Curr3 is the prev 2 bytes + current byte
    Next 3 is the look up of the next 3 bytes

    */
    // let start_buffer = *buffer_idx;

    let mut max_len = 0;
    let mut curr_dist = 0;



    if *buffer_idx + 2 >= buffer.len(){
        let curr_option = if max_len != 0 {
            Some((curr_dist, max_len))
        } else {
            None
        };
        return curr_option;
    }
    let temp_matches = _get_temp_matches_buffer(*buffer_idx, buffer);

    let next_3_bytes = (buffer[*buffer_idx], buffer[*buffer_idx + 1], buffer[*buffer_idx + 2]);

    // let temp_matches = dbg!(temp_matches);
    
    if let Some(index) = temp_matches.get(&next_3_bytes){
        // this wont work, need to find the right index
        let mut found_idx = *index;
        let start_match_idx = found_idx;
        let mut temp_buffer_idx = *buffer_idx + 2; 

        loop {
            if temp_buffer_idx >= 3 {
                let val = true_matches.entry((buffer[temp_buffer_idx-3], buffer[temp_buffer_idx-2], buffer[temp_buffer_idx-1])).or_insert(FxHashSet::default());

                val.insert(temp_buffer_idx-2);
            }
            if temp_buffer_idx < buffer.len() {
                let next_el = buffer[found_idx];
                if next_el != buffer[temp_buffer_idx] || (temp_buffer_idx - *buffer_idx >= 258){
                    break;
                }
                temp_buffer_idx += 1;
                found_idx += 1;
            } else {
                break;
            }
            // updates the max length
            if temp_buffer_idx - *buffer_idx >= max_len {
                curr_dist = *buffer_idx - start_match_idx;
                max_len = temp_buffer_idx - *buffer_idx;
            }
        }
    }

    // let true_matches = dbg!(true_matches);

    // let next_3_bytes = next_3_bytes;
    let mut to_remove = Vec::new();
    if let  Some(indices) = true_matches.get(&next_3_bytes){
        for index in indices.clone(){
            if *buffer_idx <= index || *buffer_idx - index > 32768 {
                to_remove.push(index);
                continue;
            }
            // println!("index: {}", index);
            let mut found_idx = index;
            let start_match_idx = found_idx;
            let mut temp_buffer_idx = *buffer_idx; 
            if max_len == 258{
                break;
            }
            loop {
                if temp_buffer_idx >= buffer.len() {
                    break;
                }
                if buffer[found_idx] != buffer[temp_buffer_idx] || (temp_buffer_idx - *buffer_idx >= 258){
                    break;
                }                
                if temp_buffer_idx >= 2 {
                    let key = (buffer[temp_buffer_idx-2], buffer[temp_buffer_idx-1], buffer[temp_buffer_idx]);
                    if !true_matches.contains_key(&key) {
                        true_matches.insert(key, FxHashSet::default());
                    }
                    true_matches.get_mut(&key).unwrap().insert(temp_buffer_idx-2);
                }
                temp_buffer_idx += 1;
                found_idx += 1;
            }
            // println!("curr_dist {}, best distance: {}", *buffer_idx - start_match_idx, curr_dist);
            if temp_buffer_idx - *buffer_idx > max_len {
                curr_dist = *buffer_idx - start_match_idx;
                max_len = temp_buffer_idx - *buffer_idx;
            }
        }
    }
    for index in to_remove {
        true_matches.get_mut(&next_3_bytes).unwrap().remove(&index);
    }

    let curr_option = if max_len != 0 {
        Some((curr_dist, max_len))
    } else {
        None
    };

    curr_option
}

pub fn lz77_compression(buffer: Vec<u8>, compressed: &mut Vec<bool>){

    // FxHashMap of the bytes to the previous found indices
    let mut true_matches: FxHashMap<(u8, u8, u8), FxHashSet<usize>> = FxHashMap::default();

    if buffer.len() < 3{
        for num in buffer{
            compressed.append(&mut reverse_huffman(num));
        }
        return;
    }

    let mut buffer_idx = 0;

    while buffer_idx < buffer.len(){

        if buffer_idx >= 2 {
            let val = true_matches.entry((buffer[buffer_idx-2], buffer[buffer_idx-1], buffer[buffer_idx])).or_insert(FxHashSet::default());
            val.insert(buffer_idx-2);
        }

        let matches = find_match_buffer(&buffer, &buffer_idx, &mut true_matches);
        match matches{
            Some(current_match_val) =>{
                let (distance, length) = current_match_val;

                let mut len_arr = convert_length(length);
                let mut dist_arr = convert_dist(distance);

                compressed.append(&mut len_arr);
                compressed.append(&mut dist_arr);

                buffer_idx += length;

                // output the current match
            },
            None =>{
                // output match
                let curr_byte = buffer[buffer_idx];
                let mut bool_arr = reverse_huffman(curr_byte);
                compressed.append(&mut bool_arr);
                buffer_idx += 1;
            }
        }
        // println!("buff idx: {}", buffer_idx);
    }
    
}

pub fn compress(buffer: Vec<u8>) -> Vec<u8>{
    // let test_buff = &[97, 98, 99, 100, 97, 98, 99, 100, 97, 98, 99, 100]; 
    
    let mut all_bits = Vec::new();
    
    // BFinal
    
    all_bits.push(true);
    
    // BTYPE

    all_bits.push(true);
    all_bits.push(false);

    lz77_compression(buffer, &mut all_bits);
    
    let mut eob = vec![false, false, false, false, false, false, false];
    all_bits.append(&mut eob);


    let additional_zeros = if all_bits.len() % 8 != 0 {
        8 - all_bits.len() % 8
    } else {
        0
    };

    for _ in 0..additional_zeros{
        all_bits.push(false);
    }

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
    out
}

// fn main(){
//     println!("Hello World!!!! :D");

//     env::set_var("RUST_BACKTRACE", "1");

//     let args: Vec<String> = env::args().collect();
//     let file_name : &String = &args[1];

//     let mut f = File::open(file_name).expect("couldn't open file");
//     let mut buffer = Vec::new();
//     f.read_to_end(&mut buffer).expect("couldn't read file");  

//     let out = compress(buffer);

//     let output_file_name = format!("{}.deflate", file_name);

//     let mut file = File::create(output_file_name).expect("Failed to open");
//     file.write_all(&out).expect("Couldn't write bytes");
// }