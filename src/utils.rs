use std::{cmp::Ordering, vec};
// use std::{collections::{HashSet}, env, fs::{self, File}, io::{Write, Read}};


pub const CODE_LENGTH_ORDER: [u16; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

pub const LENGTH_STARTING : [u16; 7] = [3, 11, 19, 35, 67, 131, 258];

pub const DISTANCE_CODES: [u16; 14] = [0, 5, 9, 17, 33, 65, 129, 257, 513, 1025, 2049, 4097, 8193, 16385];

pub const BITS_TO_READ: [(u16, u16); 6] = [(257, 264), (265, 268), (269, 272), (273, 276),(277, 280), (281, 284)];

pub const DISTANCE_BITS: usize = 5;

pub const CHARACTER_BOUNDS : [(u16, u16); 4] = [(0, 23), (48, 191), (192, 199), (400, 511)];// u16::from_str_radix("00110000", 2).expect("bruh");

// #[allow(const_evaluatable_unchecked)]
static HUFF_STUFF: [[bool; 9]; 143] = gen_table();
// use lazy_static::lazy_static;

// lazy_static! {
//     pub static ref HUFF_STUFF: [[bool; 9]; 143] = gen_table();
// }

pub mod huffman_old;
pub mod dynamic;
pub mod huffman;
pub mod huffman_new;


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

pub const fn get_bit_reverse(buffer : &[u8], idx : usize) -> bool {
    /*
    Returns the bit at the given index in the buffer.
    */

    // let buffer_idx : usize = idx / 8
    let byte = buffer[idx / 8];

    // go through the stuff regularly
    let bit = (byte << (idx % 8)) & 128u8 == 128u8;

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
    let mut bits = Vec::with_capacity(8); 

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

    // let mut eob  = false;
    // let mut cnt = 0;

    loop {
        let ascii = get_num_buffer(buffer, idx);

        match ascii {
            AsciiNum::Ascii(value) => {
                // println!("val: {}", value);
                if value == 256{
                    // eob = true;
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
    

    let mut bits_offset;

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
    // let mut len_num = 0;
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



pub const fn get_9_bits_reverse(buffer : &[u8], idx : usize) -> [bool; 9] {
    // This just gets the bits going from left to right
    let mut bits = [false; 9]; 
    let n = 9;
    let mut i = 0;
    while i < n{
        let bit = get_bit_reverse(buffer, idx+i);
        bits[i] = bit;
        i += 1;
    }
    // let bits = dbg!(bits);
    bits
}


const fn gen_table()-> [[bool; 9]; 143]{
    const fn gen_huff(num:u16)->[bool; 9]{
        let val: u16 = num as u16 - 144 + 400;
        let big_bytes = (val >> 8) as u8;
        let little_bytes = val as u8;
        let bruh = get_9_bits_reverse(&[big_bytes, little_bytes], 16-9);
        // panic!();
        return bruh;
    }
    let mut ans: [[bool; 9]; 143] = [[false; 9]; 143];    
    let mut start_idx = 144;
    while start_idx <=255{
        ans[start_idx - 144] = gen_huff(start_idx as u16);
        start_idx += 1;
    } 
    ans
}


fn reverse_huffman(num: u8) -> Vec<bool>{
    match num.cmp(&144){
        Ordering::Equal => {
            // panic!();
            return vec![true, true, false, false, true, false, false, false, false]
        },
        Ordering::Less => {
            let new_num = num + 0b00110000;
            return get_n_bits_reverse(&[new_num], 0, 8);
        },
        Ordering::Greater => {
            // let val: u16 = num as u16 - 144 + 400;
            // let big_bytes = (val >> 8) as u8;
            // let little_bytes = val as u8;
            // let bruh = get_n_bits_reverse(&[big_bytes, little_bytes], 16-9, 9);
            // panic!();
            return HUFF_STUFF[num as usize - 144].to_vec();
            // return bruh;
            // return HUFF_STUFF[num as usize - 144].to_vec();
        }
    }
}