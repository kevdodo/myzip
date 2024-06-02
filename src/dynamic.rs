use core::num;
use std::collections::hash_map;
use std::hash::Hash;
use std::thread::current;
use std::{cmp::Ordering, collections::HashMap, vec};
use crate::*;

use crate::DISTANCE_CODES;


pub fn get_code_length_code_matrix(buffer: &[u8], idx: &mut usize, hclen: u16) -> Vec<u16>{    
    // let alphabet : [u16; 18] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1];

    let num_codes = hclen as usize + 4;
    let mut code_lengths : Vec<u16> = Vec::new();
        
    for i in 0..num_codes{
        let num = get_num_reverse(&get_n_bits_reverse (&buffer, *idx + i*3, 3));
        code_lengths.push(num);
    }
    *idx = *idx + 3 * num_codes;
    code_lengths.to_vec()
}

pub fn convert_code_lengths_matrix(code_lengths: &Vec<u16>, alphabet: Vec<u16>) -> Vec<[u16; 100000]>{

    // (1) Count the number of codes for each code
    // length by populating an array bl_count, where bl_count[N]
    // is the number of codes of length N. Make sure to set bl_count[0] = 0.

    let mut counts: HashMap<u16, usize> = HashMap::new();

    for code_length in code_lengths{
        if *code_length != 0{
            *counts.entry(*code_length).or_insert(0) += 1;
        }
    }

    let counts = counts;

    let max_len = match counts.keys().max() {
        Some(&max) => max as usize + 1,
        None => 0, 
    };

    // println!("max len {}", max_len);
    let mut bl_count: Vec<usize> = vec![0; max_len];
    for (ch, count) in counts {
        bl_count[ch as usize] = count;
    }
    
    // bl_count = dbg!(bl_count);
    
    // (2) Find the numerical value of the smallest code for each code length with the following algorithm:

    let mut code: usize = 0;
    let mut next_code: Vec<usize> = vec![0; max_len];
    for i in 1..max_len {
        code = (code + bl_count[i-1]) << 1;
        next_code[i] = code;
    }
    // next_code = dbg!(next_code);

    // (3) Loop through the alphabet in lexicographical order and assign consecutive values starting length i

    get_code_length_matrix(&mut next_code, alphabet, &code_lengths)
    // vec![[0; 100000]]

}

// TODO: I might be cooked if you can get huffman values greater than 100000
pub fn get_code_length_matrix(next_code: &mut Vec<usize>, alphabet: Vec<u16>, code_lengths: &Vec<u16>) -> Vec<[u16; 100000]> {
    
    // code length is rows. Is this doable as with the u16's

    /*  Example codebook  (alphabet -> (length, code in number)): 
        1: (
        2,
        3,
    ),
    18: (
        1,
        0,
    ),
    0: (
        2,
        2,
    ),

    //            number
    //       max  max  max max
    // len   18   max  max max
    //       max  max   0   1

     */
    let max_len = code_lengths.iter().max();

    let max_code_length = match max_len {
        Some(val) => *val as usize,
        None => {
            panic!("Code lengths is empty")
        }
    };

    if max_code_length == 0{
        println!("Warning: Code lengths is empty !!!");
    }
    
    let mut tree: Vec<[u16; 100000]> = Vec::with_capacity(max_code_length); //vec![[u16::MAX; 100000]];
    
    for _ in 0..max_code_length+1{
        tree.push([u16::MAX; 100000]);
    }
    let num_rows = tree.len();
    
    // println!("Number of rows: {}", num_rows);

    // let alphabet = dbg!(alphabet);
    let slice = &alphabet[0..code_lengths.len()];
    let mut alphabet_sorted = Vec::from(slice).clone();
    alphabet_sorted.sort();

    

    // let code_lengths = dbg!(code_lengths);

    // create a map of where they are in the code length index
    let mut alphabet_to_idx_map = HashMap::new();

    for (index, &value) in alphabet.iter().enumerate() {
        alphabet_to_idx_map.insert(value, index);
    }

    let alphabet_to_idx_map = alphabet_to_idx_map;

    for idx in 0..alphabet_sorted.len() {
        let key = alphabet_sorted[idx];
        let index = alphabet_to_idx_map.get(&key);

        match index {
            Some(&i) => {

                let code_len = code_lengths[i];
                // Code len 0's are not used
                if code_len != 0{
                    tree[code_len as usize][next_code[code_len as usize]] = alphabet_sorted[idx];
                    // println!("asdfasdf: {} code: {}, len: {}", alphabet_sorted[idx], next_code[code_len as usize], code_len);
                    next_code[code_len as usize] += 1;
                }
            },
            None => {
                // Handle the case where the index is not found in the map
                // ...
                panic!("why tf this none")
            },
        }
    }

    tree
}

pub fn get_codes_lit_dist(code_len_tree: &Vec<[u16; 100000]>, buffer : &[u8], buffer_idx : &mut usize, num_codes:usize) -> Vec<[u16; 100000]>{
    let mut code_lengths = vec![0; num_codes as usize];

    let mut code_idx = 0;

    // println!("number of codes: {}", num_codes);

    while code_idx < num_codes{
        // TODO: this can definitely be faster ie if bit is 0, shift left, 1 is shift left and add 1
        let mut val = u16::MAX;
                
        let mut n = 0;
        // dbg!(get_n_bits_reverse(buffer, *buffer_idx, 10));
        
        while val == u16::MAX{
            let num = get_num(&get_n_bits_reverse(buffer, *buffer_idx, n));
            val = code_len_tree[n][num as usize];
            n += 1;
        }

        *buffer_idx += n-1;
        // TODO: make it a cool match statement
        if val == 18 {
            let repeats = get_num_reverse(&get_n_bits_reverse(buffer, *buffer_idx, 7)) as usize + 11;

            for i in code_idx..(code_idx+repeats as usize){
                code_lengths[i] = 0;
            }
            code_idx = code_idx + repeats;
            *buffer_idx += 7;
        }
        else if val == 17 {
            let repeats = get_num_reverse(&get_n_bits_reverse(buffer, *buffer_idx, 3)) as usize + 3;
            for i in code_idx as usize..(code_idx+repeats) as usize{
                code_lengths[i] = 0;
            }
            code_idx = code_idx + repeats;
            *buffer_idx += 3;
            // return vec![[0; 100000]];

        }
        else if val == 16{
            // return vec![[0; 100000]];

            let repeats = get_num_reverse(&get_n_bits_reverse(buffer, *buffer_idx, 2)) as usize + 3;
            for i in code_idx..code_idx+repeats as usize{
                code_lengths[i] = code_lengths[code_idx-1];
            }
            code_idx = code_idx + repeats;
            *buffer_idx += 2;

        }
        else{
            code_lengths[code_idx] = val;
            code_idx += 1;
        }
    }

    let alphabet: Vec<u16> = (0..num_codes as usize).map(|x| x as u16).collect();
    // println!("debugging code_lengths literals");
    // let code_lengths = dbg!(code_lengths);

    convert_code_lengths_matrix(&code_lengths, alphabet)
}


// File data was added to support multiple blocks
pub fn decode_dynamic(literal_tree: Vec<[u16; 100000]>, distance_tree: Vec<[u16; 100000]>, buffer : &[u8], buffer_idx : &mut usize, file_data: &mut Vec<u8>){
    
    let mut curr_byte = 0;
    while curr_byte != 256 { // && curr_idx < buffer.len() * 8
        let mut val = u16::MAX;
                
        let mut n = 0;
        // dbg!(get_n_bits_reverse(buffer, *buffer_idx, 10));
        
        while val == u16::MAX{
            let num = get_num(&get_n_bits_reverse(buffer, *buffer_idx, n));
            val = literal_tree[n][num as usize];
            n += 1;
        }
        *buffer_idx += n - 1;
        // println!("number: {} ", val);
        curr_byte = val;
        match val.cmp(&256) {
            // end of block
            Ordering::Equal => (),
            // length val
            Ordering::Greater => {
                curr_byte = val;
                let len_dist = process_length(val, buffer,
                     buffer_idx, &distance_tree);

                match len_dist {
                    AsciiNum::LenDist(value) => {
                        decode_lz77(value, file_data);
                        // println!("length: {}, distance: {}", value.0, value.1);
                    }
                    AsciiNum::Ascii(value) => {
                        // println!("val: {}", value);
                        panic!("this should not be an ascii thing {}", value);
                    }
                }
            },
            // push that character to bytes, the value is j the byte
            Ordering::Less => file_data.push(curr_byte as u8)
        }
    }
}
pub fn process_length(num: u16, buffer: &[u8], idx: &mut usize, distance_tree: &Vec<[u16; 100000]>) -> AsciiNum {
    // get_length
    // get_distance

    let mut cnt = 0;
    let mut len_bits_to_read = 0;

    for (x, y) in BITS_TO_READ{
        if num >= x && num <= y {
            len_bits_to_read = cnt;
        }
        cnt += 1;
    } 

    let additional_bits = get_n_bits_reverse(buffer, *idx, len_bits_to_read);

    let mut length = get_lower_bound(len_bits_to_read, num) + get_num_reverse(&additional_bits);

    if num == 285{
        len_bits_to_read = 0;
        length = 258;
    }

    let mut distance_num = u16::MAX;
    let mut n = 0;

    *idx = *idx + len_bits_to_read;

    while distance_num == u16::MAX{
        let num = get_num(&get_n_bits_reverse(buffer, *idx, n));
        distance_num = distance_tree[n][num as usize];
        n += 1;
    }
    // dbg!(get_n_bits_reverse(buffer, *idx, n));
    // println!("distance stuff {}", distance_num);

    // distance bits are 5 bits
    // let dist_bits = dbg!(get_n_bits_reverse(buffer, *idx+len_bits_to_read, DISTANCE_BITS));
    // let dist_num = dbg!(get_num_reverse(&dist_bits));

    *idx += n - 1;
    // panic!("asdfasdf");

    if distance_num > 29{
        panic!("Additional distance somehow over 29")
    }
    if distance_num < 4{
        // *idx = *idx + len_bits_to_read;
        return AsciiNum::LenDist((length, distance_num + 1));
    }

    let dist_bits_to_read = ((distance_num - 4) / 2 + 1) as usize;
    // println!("num dist bits: {}", dist_bits_to_read);

    let distance_lower = (distance_num % 2) * 2u16.pow(dist_bits_to_read as u32) + DISTANCE_CODES[dist_bits_to_read as usize];
    // println!("dist lower : {}", distance_lower);
    // println!("dist num: {}", distance_num);

    // let distance_lower = DISTANCE_CODES[dist_bits_to_read as usize] + ;
    let additional_distance = get_num_reverse(&get_n_bits_reverse(buffer, *idx, dist_bits_to_read));

    *idx += dist_bits_to_read;

    return AsciiNum::LenDist((length, distance_lower + additional_distance));

}
// /*
// // Alphabet is just going to be the numbers ie. 0-HLIT+257, HDIST+1
// pub fn convert_code_lengths (code_lengths: &[u16], alphabet: &[u16]) -> HashMap<u16, (u16, usize)>{

//     // (1) Count the number of codes for each code
//     // length by populating an array bl_count, where bl_count[N]
//     // is the number of codes of length N. Make sure to set bl_count[0] = 0.

//     let mut counts: HashMap<u16, usize> = HashMap::new();

//     for code_length in code_lengths{
//         if *code_length != 0{
//             *counts.entry(*code_length).or_insert(0) += 1;
//         }
//     }

//     let max_len = match counts.values().max() {
//         Some(&max) => max as usize + 1,
//         None => 0, 
//     };

//     println!("max len {}", max_len);

//     let mut bl_count: Vec<usize> = vec![0; max_len];
//     for (ch, count) in counts {
//         bl_count[ch as usize] = count;
//     }    
    
//     // (2) Find the numerical value of the smallest code for each code length with the following algorithm:

//     let mut code: usize = 0;
//     let mut next_code: Vec<usize> = vec![0; max_len];
//     for i in 1..max_len {
//         code = (code + bl_count[i-1]) << 1;
//         next_code[i] = code;
//     }

//     let mut codebook: HashMap<u16, (u16, usize)> = HashMap::new();

//     // (3) Loop through the alphabet in lexicographical order and assign consecutive values starting length i

//     for idx in 0..alphabet.len() {
//         let code_len = code_lengths[idx];
//         if code_len != 0{
//             codebook.insert(alphabet[idx], (code_len, next_code[code_len as usize]));
//             next_code[code_len as usize] += 1;
//         }
//     }
//     codebook
// }
// */

// pub fn get_code_length_code_matrix(buffer: &[u8], idx: &mut usize, hclen: u16) -> Vec<u16>{    
//     // let alphabet : [u16; 18] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1];

//     let num_codes = hclen as usize + 4;
//     let mut code_lengths : Vec<u16> = Vec::new();
        
//     for i in 0..num_codes{
//         let num = get_num_reverse(&get_n_bits_reverse (&buffer, *idx + i*3, 3));
//         code_lengths.push(num);
//     }
//     *idx = *idx + 3 * num_codes;
//     code_lengths.to_vec()
// }

// pub fn convert_code_lengths_matrix (code_lengths: &Vec<u16>, alphabet: Vec<u16>) -> Vec<[u16; 100000]>{

//     // (1) Count the number of codes for each code
//     // length by populating an array bl_count, where bl_count[N]
//     // is the number of codes of length N. Make sure to set bl_count[0] = 0.

//     let mut counts: HashMap<u16, usize> = HashMap::new();

//     for code_length in code_lengths{
//         if *code_length != 0{
//             *counts.entry(*code_length).or_insert(0) += 1;
//         }
//     }

//     let max_len = match counts.values().max() {
//         Some(&max) => max as usize + 1,
//         None => 0, 
//     };

//     // println!("max len {}", max_len);

//     let mut bl_count: Vec<usize> = vec![0; 2 *max_len];
//     for (ch, count) in counts {
//         bl_count[ch as usize] = count;
//     }
    
//     bl_count = dbg!(bl_count);
    
//     // (2) Find the numerical value of the smallest code for each code length with the following algorithm:

//     let mut code: usize = 0;
//     let mut next_code: Vec<usize> = vec![0; 2 * max_len];
//     for i in 1..max_len {
//         code = (code + bl_count[i-1]) << 1;
//         next_code[i] = code;
//     }
//     next_code = dbg!(next_code);

//     // (3) Loop through the alphabet in lexicographical order and assign consecutive values starting length i

//     get_code_length_matrix(&mut next_code, alphabet, &code_lengths)
//     // vec![[0; 100000]]

// }

// // TODO: I might be cooked if you can get huffman values greater than 100000
// pub fn get_code_length_matrix(next_code: &mut Vec<usize>, alphabet: Vec<u16>, code_lengths: &Vec<u16>) -> Vec<[u16; 100000]> {
    
//     // code length is rows. Is this doable as with the u16's

//     /*  Example codebook  (alphabet -> (length, code in number)): 
//         1: (
//         2,
//         3,
//     ),
//     18: (
//         1,
//         0,
//     ),
//     0: (
//         2,
//         2,
//     ),

//     //            number
//     //       max  max  max max
//     // len   18   max  max max
//     //       max  max   0   1

//      */
//     let max_len = code_lengths.iter().max();

//     let mut max_code_length = 0;

//     dbg!(max_len);

//     match max_len {
//         Some(val) => {
//             max_code_length = *val as usize;
//         }
//         None =>{
//             panic!("Code lengths is empty")
//         }
//     }

//     if max_code_length == 0{
//         println!("Warning: Code lengths is empty !!!");
//     }
    
//     let mut tree: Vec<[u16; 100000]> = Vec::with_capacity(max_code_length); //vec![[u16::MAX; 100000]];
    
//     for _ in 0..max_code_length+1{
//         tree.push([u16::MAX; 100000]);
//     }
//     let num_rows = tree.len();
    
//     println!("Number of rows: {}", num_rows);

//     // let alphabet = dbg!(alphabet);
//     let slice = &alphabet[0..code_lengths.len()];
//     let mut alphabet_sorted = Vec::from(slice).clone();
//     alphabet_sorted.sort();

//     // let code_lengths = dbg!(code_lengths);

//     // create a map of where they are in the code length index
//     let mut alphabet_to_idx_map = HashMap::new();


//     for (index, &value) in alphabet.iter().enumerate() {
//         alphabet_to_idx_map.insert(value, index);
//     }

//     let alphabet_to_idx_map = dbg!(alphabet_to_idx_map);
//     let alphabet = dbg!(alphabet);

//     // if alphabet.len() > code_lengths.len(){
//     //     panic!("noooooo")
//     // }


//     for idx in 0..alphabet_sorted.len() {
//         let key = alphabet_sorted[idx];
//         let index = alphabet_to_idx_map.get(&key);

//         match index {
//             Some(&i) => {

//                 let code_len = code_lengths[i];
//                 // Code len 0's are not used
//                 if code_len != 0{
//                     tree[code_len as usize][next_code[code_len as usize]] = alphabet_sorted[idx];
//                     next_code[code_len as usize] += 1;
//                 }
//             },
//             None => {
//                 // Handle the case where the index is not found in the map
//                 // ...
//                 panic!("why tf this none")
//             },
//         }
//     }

//     tree
// }

// pub fn get_codes_lit_dist(code_len_tree: &Vec<[u16; 100000]>, buffer : &[u8], buffer_idx : &mut usize, num_codes:usize) -> Vec<[u16; 100000]>{
//     let mut code_lengths = vec![0; num_codes as usize];

//     let mut code_idx = 0;

//     println!("number of codes: {}", num_codes);

//     while code_idx < num_codes{
//         // TODO: this can definitely be faster ie if bit is 0, shift left, 1 is shift left and add 1
//         let mut val = u16::MAX;
                
//         let mut n = 0;
//         // dbg!(get_n_bits_reverse(buffer, *buffer_idx, 10));
        
//         while val == u16::MAX{
//             let num = get_num(&get_n_bits_reverse(buffer, *buffer_idx, n));
//             val = code_len_tree[n][num as usize];
//             n += 1;
//         }

//         *buffer_idx += n-1;
//         // TODO: make it a cool match statement
//         if val == 18 {
//             let repeats = get_num_reverse(&get_n_bits_reverse(buffer, *buffer_idx, 7)) as usize + 11;

//             for i in code_idx..(code_idx+repeats as usize){
//                 code_lengths[i] = 0;
//             }
//             code_idx = code_idx + repeats;
//             *buffer_idx += 7;
//         }
//         else if val == 17 {
//             let repeats = get_num_reverse(&get_n_bits_reverse(buffer, *buffer_idx, 3)) as usize + 3;
//             for i in code_idx as usize..(code_idx+repeats) as usize{
//                 code_lengths[i] = 0;
//             }
//             code_idx = code_idx + repeats;
//             *buffer_idx += 3;
//             // return vec![[0; 100000]];

//         }
//         else if val == 16{
//             // return vec![[0; 100000]];

//             let repeats = get_num_reverse(&get_n_bits_reverse(buffer, *buffer_idx, 2)) as usize + 3;
//             for i in code_idx..code_idx+repeats as usize{
//                 code_lengths[i] = code_lengths[code_idx-1];
//             }
//             code_idx = code_idx + repeats;
//             *buffer_idx += 2;

//         }
//         else{
//             code_lengths[code_idx] = val;
//             code_idx += 1;
//         }
//     }

//     let alphabet: Vec<u16> = (0..num_codes as usize).map(|x| x as u16).collect();
//     println!("debugging code_lengths literals");
//     let code_lengths = dbg!(code_lengths);

//     convert_code_lengths_matrix(&code_lengths, alphabet)
//     // vec![[0; 100000]]
// }


// pub fn decode_dynamic(literal_tree: Vec<[u16; 100000]>, distance_tree: Vec<[u16; 100000]>, buffer : &[u8], buffer_idx : &mut usize) -> Vec<u8>{
    
//     let mut bytes = Vec::new();

//     let mut curr_byte = 0;
//     while curr_byte != 256 { // && curr_idx < buffer.len() * 8
//         let mut val = u16::MAX;
                
//         let mut n = 0;
//         // dbg!(get_n_bits_reverse(buffer, *buffer_idx, 10));
        
//         while val == u16::MAX{
//             let num = get_num(&get_n_bits_reverse(buffer, *buffer_idx, n));
//             val = literal_tree[n][num as usize];
//             n += 1;
//         }
//         *buffer_idx += n - 1;
        
//         curr_byte = val;
//         match val.cmp(&256) {
//             // end of block
//             Ordering::Equal => (),
//             // length val
//             Ordering::Greater => {
//                 curr_byte = val;
//                 let len_dist = dbg!(process_length(val, buffer,
//                      buffer_idx, n, &distance_tree));

//                 match len_dist {
//                     AsciiNum::LenDist(value) => {
//                         decode_lz77(value, &mut bytes);
//                     }
//                     AsciiNum::Ascii(value) => {
//                         println!("val: {}", value);
//                         panic!("this should not be an ascii thing {}", value);
//                     }
//                 }
//             },
//             // push that character to bytes, the value is j the byte
//             Ordering::Less => bytes.push(curr_byte as u8)
//         }
        
//         println!("bytes:");
//         for i in 0..bytes.len(){
//             println!(" {} ", bytes[i]);
//         }
//     }
//     bytes
// }
// pub fn process_length(num: u16, buffer: &[u8], idx: &mut usize, bits_offset: usize, distance_tree: &Vec<[u16; 100000]>) -> AsciiNum {
//     // get_length
//     // get_distance

//     let mut cnt = 0;
//     let mut len_bits_to_read = 0;

//     for (x, y) in BITS_TO_READ{
//         if num >= x && num <= y {
//             len_bits_to_read = cnt;
//         }
//         cnt += 1;
//     } 

//     let additional_bits = get_n_bits_reverse(buffer, *idx + bits_offset, len_bits_to_read);

//     let length = get_lower_bound(len_bits_to_read, num) + get_num(&additional_bits);


//     let mut val = u16::MAX;
//     let mut n = 0;
//     while val != u16::MAX{
//         let num = get_num_reverse(&get_n_bits_reverse(buffer,*idx + bits_offset, n));
//         val = distance_tree[n][num as usize];
//         n += 1;
//     }
//     println!("distance value: {}", val);

//     // distance bits are 5 bits
//     let dist_bits = get_n_bits_reverse(buffer, *idx+len_bits_to_read, DISTANCE_BITS);
//     let dist_num = get_num(&dist_bits);

//     if dist_num > 29{
//         panic!("Additional distance somehow over 29")
//     }
//     if dist_num < 4{
//         // *idx = *idx  + len_bits_to_read + DISTANCE_BITS;
//         return AsciiNum::LenDist((length, dist_num + 1));
//     }

//     let dist_bits_to_read = ((num - 4) / 2 + 1) as usize;
//     let distance_lower = DISTANCE_CODES[dist_bits_to_read as usize];
//     let additional_distance = get_num(&get_n_bits_reverse(buffer, *idx +  DISTANCE_BITS, dist_bits_to_read));

//     *idx = *idx +  len_bits_to_read + DISTANCE_BITS + dist_bits_to_read;

//     return AsciiNum::LenDist((length, distance_lower + additional_distance));

// }
