use core::num;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, thread::panicking, vec};
use std::{collections::{HashSet}, env, fs::{self, File}, io::{Write, Read}};


pub const CODE_LENGTH_ORDER: [u16; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

pub const LENGTH_STARTING : [u16; 7] = [3, 11, 19, 35, 67, 131, 258];

pub const DISTANCE_CODES: [u16; 14] = [0, 5, 9, 17, 33, 65, 129, 257, 513, 1025, 2049, 4097, 8193, 16385];

pub const BITS_TO_READ: [(u16, u16); 6] = [(257, 264), (265, 268), (269, 272), (273, 276),(277, 280), (281, 284)];

pub const DISTANCE_BITS: usize = 5;

pub const CHARACTER_BOUNDS : [(u16, u16); 4] = [(0, 23), (48, 191), (192, 199), (400, 511)];// u16::from_str_radix("00110000", 2).expect("bruh");



#[derive(Debug)]
pub enum AsciiNum {
    Ascii(u16),
    LenDist((u16, u16))
}

pub fn get_bit(buffer : &[u8], idx : usize) -> bool {
    /*
    Returns the bit at the given index in the buffer.
    */

    // let buffer_idx : usize = idx / 8
    let byte = buffer[idx / 8];
    
    // println!("byte: {:08b}", byte);

    // bits are stored in reverse stream order, 
    let bit = (byte >> ((idx) % 8)) & 1;

    // println!("Byte: {:08b}, Bit: {}, num {}", byte, bit, byte);
    1 == bit
}

pub fn get_num(bits: &Vec<bool>) -> u16{
    let mut num :u16 = 0;
    // let bits = dbg!(bits);

    for i in 1..(bits.len()+1){
        num += ((bits[bits.len() - i] as u16) * 2u16.pow((i-1) as u32)) as u16;
    }
    num
}

pub fn get_num_reverse(bits: &Vec<bool>) -> u16{
    let mut num :u16 = 0;
    // let bits = dbg!(bits);

    for i in 0..bits.len(){
        num += ((bits[i] as u16) * 2u16.pow((i) as u32)) as u16;
    }
    num
}

pub fn get_bit_reverse(buffer : &[u8], idx : usize) -> bool {
    /*
    Returns the bit at the given index in the buffer.
    */

    // let buffer_idx : usize = idx / 8
    let byte = buffer[idx / 8];

    // go through the stuff regularly
    let bit = (byte << (idx % 8)) & 2u8.pow(7) == 2u8.pow(7);

    // println!("{:08b}", 2u8.pow(7));

    // println!("Byte: {:08b}, Bit: {}, num {}", byte, bit, byte);
    bit
}

pub fn get_n_bits_regular(buffer : &[u8], idx : usize, n : usize) -> Vec<bool> {
    /*
    Returns the bit at the given index in the buffer.
    */
    let mut bits = Vec::new(); 

    for i in 0..n{
        let bit = get_bit(buffer, idx+i);
        bits.push(bit);
    }
    bits
}


// RENAME TO GET NEXT BITS
pub fn get_n_bits_reverse(buffer : &[u8], idx : usize, n : usize) -> Vec<bool> {
    // This just gets the bits going from left to right
    let mut bits = Vec::new(); 

    for i in 0..n{
        let bit = get_bit_reverse(buffer, idx+i);
        bits.push(bit);
    }
    // let bits = dbg!(bits);
    bits
}


pub fn decode_lz77(value : (u16, u16), decoded : &mut Vec<u8>) {
    // length is first value
    let value = value;
    let mut curr_pointer = decoded.len() - value.1 as usize;
    // let decoded = dbg!(decoded);
    for _i in 0..(value.0) as usize{
        let character = decoded[curr_pointer];
        decoded.push(character);
        curr_pointer += 1;
    }
}

// TODO: change index to be a reference so that we can just
// keep calling get_num_buffer

pub fn decode_deflate(buffer : &[u8], idx: &mut usize, file_data: &mut Vec<u8>){

    let mut eob  = false;
    let mut cnt = 0;

    while !eob{
        let ascii = get_num_buffer(buffer, idx);

        match ascii {
            AsciiNum::Ascii(value) => {
                // println!("val: {}", value);
                if value == 256{
                    eob = true;
                    break;
                }
                file_data.push(value as u8);
            }
            AsciiNum::LenDist(value) => {
                let value = value;
                //decoded = dbg!(decoded);
                //panic!("ermn");
                decode_lz77(value, file_data);
                // cnt += 1;
                // if cnt >=10{
                //     break;
                // }
            }
        }
        // if decoded.len() > 26{
        //     println!("WRONG: ");
        //     let ascii = dbg!(ascii);
        //     println!("decoded length: {}", decoded.len());
        //     dbg!(decoded);
        //     panic!();
        // }
        // println!("curr_idx : {}", idx);
        // decoded = dbg!(decoded);
        // dbg!(ascii);
    }
}

pub fn get_num_buffer(buffer: &[u8], idx: &mut usize) -> AsciiNum {
    /*
    Gets the number at a given idx of the buffer

    */

    let mut bits = get_n_bits_reverse(buffer, *idx, 7);
    let mut num = get_num(&bits);

    let bound = CHARACTER_BOUNDS[0].1;
    // println!("nummm {}", num);
    // println!("bound {}", bound);
    

    let mut bits_offset = 0;

    if num <= bound {
        num = num + 256;
        bits_offset = 7;
    }

    else{
        bits = get_n_bits_reverse(buffer, *idx, 8);
        num = get_num(&bits);
        bits_offset = 8;

        if num >= CHARACTER_BOUNDS[1].0 && num <= CHARACTER_BOUNDS[1].1 {
            num -= CHARACTER_BOUNDS[1].0; //u16::from_str_radix("00110000", 2).expect("bruh");
        } else if num >= CHARACTER_BOUNDS[2].0 && num <= CHARACTER_BOUNDS[2].1{    
            num -= CHARACTER_BOUNDS[2].0; //u16::from_str_radix("11000000", 2).expect("bruh");
            num = num + 280;  
        } else {
            bits = get_n_bits_reverse(buffer, *idx, 9);

            num = get_num(&bits);
            bits_offset = 9;
        
            if num >= CHARACTER_BOUNDS[3].0 && num <= CHARACTER_BOUNDS[3].1 {
                num -= CHARACTER_BOUNDS[3].0; //u16::from_str_radix("110010000", 2).expect("bruh");
                num = num + 144;
                
            }
            else{
                panic!("not a valid number for the buffer");
            }
        }
    }

    // println!("index:::  {}", idx);
    
    // if *idx == 4018{
    //     println!("num:::  {}", num);
    // }
    // println!("number before length handling {}", num);
    // Handle the case the number is a length
    
    if num < 257{
        *idx = *idx + bits_offset;
        // println!("whatttt {}", idx);
        return AsciiNum::Ascii(num);
    }

    // Must be a distance length pair
    let mut cnt = 0;
    let mut len_bits_to_read = 0;

    for (x, y) in BITS_TO_READ{
        if num >= x && num <= y {
            len_bits_to_read = cnt;
        }
        cnt += 1;
    }

    // todo: find a closed form and make it more beautiful
    let additional_bits = get_n_bits_reverse(buffer, *idx+bits_offset, len_bits_to_read);
    let mut length = get_lower_bound(len_bits_to_read, num) + get_num_reverse(&additional_bits);

    if num == 285{
        // *idx = *idx + bits_offset;
        length = 258;
        len_bits_to_read = 0;
        // panic!("holddddd upppppp")
    }

    // println!("num: {}", num);
    // println!("len_bits_to_read: {}", len_bits_to_read);
    // println!("additional_bits: {}", get_num_reverse(&additional_bits));
    // println!("length: {}", length);
    // dbg!(additional_bits);



    // distance bits are 5 bits
    let dist_bits = get_n_bits_reverse(buffer, *idx+bits_offset+len_bits_to_read, DISTANCE_BITS);
    let dist_num = get_num(&dist_bits);

    if dist_num > 29{
        panic!("Additional distance somehow over 29")
    }
    if dist_num < 4{
        *idx = *idx + bits_offset + len_bits_to_read + DISTANCE_BITS;
        return AsciiNum::LenDist((length, dist_num + 1));
    }

    let dist_bits_to_read = ((dist_num - 4) / 2 + 1) as usize;

    let distance_lower_for_bits = DISTANCE_CODES[dist_bits_to_read as usize];

    let distance_lower = (dist_num % 2) * 2u16.pow(dist_bits_to_read as u32) + distance_lower_for_bits;

    let total_distance = get_num_reverse(&get_n_bits_reverse(buffer, *idx + bits_offset + len_bits_to_read +  DISTANCE_BITS, dist_bits_to_read)) + distance_lower;


    *idx = *idx + bits_offset + len_bits_to_read + DISTANCE_BITS + dist_bits_to_read;

    return AsciiNum::LenDist((length, total_distance));

}


pub fn get_lower_bound(bits_to_read: usize, num: u16) -> u16{
    let starting_point = LENGTH_STARTING[bits_to_read]; 
    if num < 257{
        panic!("Num {} is below 257", num)
    }
    if num < 265{
        return num - 257 + 3;
    }
    let ans = ((num - 265) % 4) * 2u16.pow(bits_to_read as u32) + starting_point;

    // starting_point + bits_to_read;
    // 278 - 265 = 13 % 4 = 1 * 2 ** 4 = 16 + 67
    ans

}

fn number_to_bool_vec(mut num: usize, num_bits: usize) -> Vec<bool> {
    if num == 0{
        return vec![false; num_bits];
    }
    let mut vec = Vec::new();
    while num > 0 {
        vec.push(num % 2 != 0);
        num /= 2;
    }
    while vec.len() < num_bits{
        vec.push(false);
    }
    // vec.reverse();
    vec
}


fn reverse_huffman_lengths(length: usize) -> Vec<bool>{
    assert!(length > 256);
    // println!("nummm, {}", length);
    // println!("yuhhhh");
    match length.cmp(&280) {
        Ordering::Equal => {
            return vec![true, true, false, false, false, false, false, false]
        },
        Ordering::Less => {
            let new_num = (length - 256) as u8;
            return get_n_bits_reverse(&[new_num], 1, 7);
        },
        Ordering::Greater => {
            let new_num = (length - 280) as u8 + 0b11000000;

            let ahh = get_n_bits_reverse(&[new_num], 0, 8);
            return ahh;

        },
    }
}
pub fn convert_length(length: usize) -> Vec<bool>{
    let mut len_num = 0;
    let mut num_bits = 0; 

    // println!("Length!!!: {}", length);

    if length == 258{
        return reverse_huffman_lengths(285);
    }
    if length < 11{
        return reverse_huffman_lengths(257 + length - 3);
    }

    for (idx, length_bound) in LENGTH_STARTING.iter().enumerate(){
        if length < *length_bound as usize {
            num_bits = idx - 1;
            break;
        }
    }
    // println!("num bits: {}", num_bits);

    // println!("length {} num bits: {}", length, num_bits);

    if num_bits != 0{
        let num_steps = (length - LENGTH_STARTING[num_bits] as usize) / 2u16.pow(num_bits as u32) as usize;
        let num = num_steps + 265 + 4 * (num_bits - 1);
        let remaining = length - (num_steps * (2u16.pow(num_bits as u32) as usize) + LENGTH_STARTING[num_bits] as usize);
        // println!("number: {}, num_steps: {}, remaining: {}", num, num_steps, remaining);
        let mut remaining_bool = number_to_bool_vec(remaining, num_bits);
    
        let mut ans = reverse_huffman_lengths(num);
        ans.append(&mut remaining_bool);
        return ans;
    }
    else{
        panic!("Shoudl have handled in 11 case");
    }
}

pub fn convert_dist(distance: usize) -> Vec<bool>{
    let mut num_bits = 0; 

    for (idx, distance_bound) in DISTANCE_CODES.iter().enumerate(){
        if distance < *distance_bound as usize {
            num_bits = idx -1;
            break;
        }
    }
    if distance >= 16385{
        num_bits = 13;
    }

    if num_bits != 0{
        let num_steps = (distance - DISTANCE_CODES[num_bits] as usize) / 2u16.pow(num_bits as u32) as usize;
        let num = num_steps + 2*(num_bits + 1);
        let remaining = distance - (num_steps * (2u16.pow(num_bits as u32) as usize) + DISTANCE_CODES[num_bits] as usize);
        // println!("number: {}, num_steps: {}, remaining: {}", num, num_steps, remaining);
        let mut remaining_bool = number_to_bool_vec(remaining, num_bits);
    
        let mut ans = number_to_bool_vec(num, 5);
        ans.reverse();
        ans.append(&mut remaining_bool);
        return ans;
    }
    else{
        let mut ans = number_to_bool_vec(distance-1, 5);
        ans.reverse();
        return ans;
    }
}


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

    let mut max_code_length = 0;

    // dbg!(max_len);

    match max_len {
        Some(val) => {
            max_code_length = *val as usize;
        }
        None =>{
            panic!("Code lengths is empty")
        }
    }

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

fn main(){

}