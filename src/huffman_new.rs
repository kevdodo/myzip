
use fxhash::FxHashMap;

// use std::io::Read;
pub const LENGTH_STARTING : [u16; 6] = [3, 11, 19, 35, 67, 131];

pub const DISTANCE_CODES: [u16; 14] = [0, 5, 9, 17, 33, 65, 129, 257, 513, 1025, 2049, 4097, 8193, 16385];

// mod huffman_old;
use crate::*;


pub const MAX_MATCH_LEN: usize = 258;
pub const LZ_DICT_SIZE: usize = 32_768;
pub const LZ_DICT_FULL_SIZE: usize = LZ_DICT_SIZE + MAX_MATCH_LEN - 1 + 1;
const LZ_DICT_SIZE_MASK: usize = (LZ_DICT_SIZE as u32 - 1) as usize;

// [miniz_oxide/miniz_oxide/src/deflate/core.rs:1297:9] self.max_probes = [
//     12,
//     4,
// ]

#[derive(Debug)]
pub struct HashBuffers {
    // contains character
    pub dict: [u8; LZ_DICT_FULL_SIZE],
    // next "hash" for current position
    pub next: [usize; LZ_DICT_SIZE],
    // represents hash chain
    pub hash: [usize; LZ_DICT_SIZE],
}

impl HashBuffers {
    #[inline]
    pub fn reset(&mut self) {
        *self = HashBuffers::default();
    }
}

impl Default for HashBuffers {
    fn default() -> HashBuffers {
        HashBuffers {
            dict: [0; LZ_DICT_FULL_SIZE],
            next: [0; LZ_DICT_SIZE],
            hash: [0; LZ_DICT_SIZE],
        }
    }
}

struct RollingHash {
    hash: u32,
    chars: [u8; 3],
    length: usize,
}

impl RollingHash {
    const BASE: u32 = 4;
    const MOD: u32 = 4096;

    fn new() -> Self {
        RollingHash {
            hash: 0,
            chars: [0; 3],
            length: 0,
        }
    }

    fn add_char(&mut self, ch: u8) {
        if self.length == 3 {
            self.remove_char();
        }
        self.chars[self.length] = ch;
        self.length += 1;
        self.hash = (self.hash * Self::BASE + ch as u32) % Self::MOD;
    }

    fn remove_char(&mut self) {
        let first_char_val = (self.chars[0] as u32 * Self::BASE.pow(2)) % Self::MOD;
        self.hash = (self.hash + Self::MOD - first_char_val) % Self::MOD;
        self.chars.rotate_left(1);
        self.length -= 1;
    }

    fn add_trigram(&mut self, tuple: (u8, u8, u8)){
        self.add_char(tuple.0);
        self.add_char(tuple.1);
        self.add_char(tuple.2);
    }
}


fn _get_temp_matches_buffer(buffer_idx: usize, buffer: &Vec<u8>) -> [(u8, u8, u8, usize); 3]{
    // let mut temp_matches = FxHashMap::default();
    let mut chars =  [(0, 0, 0, 0); 3];

    if buffer_idx as i32 - 3 >= 0 {
        // temp_matches.insert((buffer[buffer_idx-3], buffer[buffer_idx-2], buffer[buffer_idx-1]), buffer_idx-3);
        chars[0] = (buffer[buffer_idx-3], buffer[buffer_idx-2], buffer[buffer_idx-1], buffer_idx-3);
    }
    if buffer_idx as i32 - 2 >= 0 {
        // temp_matches.insert((buffer[buffer_idx-2], buffer[buffer_idx-1], buffer[buffer_idx-2]), buffer_idx - 2);
        chars[1] = (buffer[buffer_idx-2], buffer[buffer_idx-1], buffer[buffer_idx-2], buffer_idx - 2);
    }
    if buffer_idx > 0{
        // temp_matches.insert((buffer[buffer_idx-1], buffer[buffer_idx-1], buffer[buffer_idx-1]), buffer_idx-1);
        chars[1] = (buffer[buffer_idx-1], buffer[buffer_idx-1], buffer[buffer_idx-1], buffer_idx-1);
    }
    // temp_matches
    chars
}

fn find_match_rle(buffer: &Vec<u8>, buffer_idx: &usize, 
    next_3_bytes: (u8, u8, u8), temp_hash: &mut RollingHash,
    true_matches: &mut HashBuffers, 
    temp_matches: [(u8, u8, u8, usize); 3], max_len: &mut usize, curr_dist: &mut usize){

    
    for i in 0..3{
        if (temp_matches[i].0, temp_matches[i].1, temp_matches[i].2) == next_3_bytes{
            let mut found_idx = temp_matches[i].3;
            let start_match_idx = found_idx;
            let mut temp_buffer_idx = *buffer_idx + 2; 
    
            let mut max_temp_buffer_idx = temp_buffer_idx;

                if temp_buffer_idx >= 3 {
                    let val = (buffer[temp_buffer_idx-3], buffer[temp_buffer_idx-2], buffer[temp_buffer_idx-1]);
                    // val.push_back(temp_buffer_idx-2);
                    
                    // can be faster
                    temp_hash.add_char(val.0);
                    temp_hash.add_char(val.1);
                    temp_hash.add_char(val.2);
                    
                    // checks if it has been added already 
                    if true_matches.hash[temp_hash.hash as usize] != (temp_buffer_idx - 3) & LZ_DICT_SIZE_MASK {
                        true_matches.next[(temp_buffer_idx - 3) & LZ_DICT_SIZE_MASK] = true_matches.hash[temp_hash.hash as usize];
                        true_matches.hash[temp_hash.hash as usize] = (temp_buffer_idx - 3) & LZ_DICT_SIZE_MASK;
                    }
                }
                if temp_buffer_idx < buffer.len() {
                    max_temp_buffer_idx = max_temp_buffer_idx.max(temp_buffer_idx);
    
                    let next_el = buffer[found_idx];
                    if next_el != buffer[temp_buffer_idx] || (temp_buffer_idx - *buffer_idx >= 258){
                        break;
                    }
                    temp_buffer_idx += 1;
                    found_idx += 1;
                    if temp_buffer_idx - *buffer_idx >= *max_len {
                        *curr_dist = *buffer_idx - start_match_idx;
                        *max_len = temp_buffer_idx - *buffer_idx;
                    }
                } else {
                    break;
                }
                // updates the max length
                
        }
    }
}

fn find_match_buffer_rolling(buffer: &Vec<u8>, buffer_idx: &usize, true_matches: &mut HashBuffers) -> Option<(usize, usize)>{

    /*
    
    Curr3 is the prev 2 bytes + current byte
    Next 3 is the look up of the next 3 bytes

    */
    // let start_buffer = *buffer_idx;

    

    let look_back_window = if *buffer_idx < LZ_DICT_SIZE{
        0
    } else {
        *buffer_idx-LZ_DICT_SIZE
    };

    let look_ahead_window = buffer.len().min(*buffer_idx+MAX_MATCH_LEN);

    let lz_dict_window = (look_back_window, look_ahead_window);
    let window = &buffer[lz_dict_window.0..lz_dict_window.1];

    
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

    let mut temp_hash = RollingHash::new();
    temp_hash.add_trigram(next_3_bytes);

    find_match_rle(buffer, buffer_idx, next_3_bytes, &mut temp_hash, true_matches, temp_matches, &mut max_len, &mut curr_dist);

    // reset the temp_hash to be the next 3 bytes
    temp_hash.add_trigram(next_3_bytes);

    let mut index = true_matches.hash[temp_hash.hash as usize];
    let mut max_temp_buffer_idx = *buffer_idx;

    // FIX PARAMETER
    let mut num_probes_left = 6; 

    'index_loop: while index != 0{
        // println!("curr idx: {}, {}", index, buffer[index as usize] as char);
        'found: loop {
            num_probes_left -= 1;
            if num_probes_left == 0{
                let curr_option = if max_len >= 3 {
                    Some((curr_dist, max_len))
                } else {
                    None
                };
                return curr_option
            }

            for _ in 0..3{
                let index1 = index & LZ_DICT_SIZE_MASK;
                let index2 = (index + 1) & LZ_DICT_SIZE_MASK;
                let index3 = (index + 2) & LZ_DICT_SIZE_MASK;
            
                let element1 = window[index1];
                let element2 = window[index2];
                let element3 = window[index3];
            
                if element1 == buffer[*buffer_idx] && element2 == buffer[*buffer_idx + 1] && element3 == buffer[*buffer_idx + 2] {
                    break 'found;
                }
                let idx = index & LZ_DICT_SIZE_MASK;

                index = true_matches.next[idx];
            }
        }

        if *buffer_idx <= index.into() {
            let next_idx = true_matches.next[index as usize];
            index = next_idx;
            continue;
        }
        if *buffer_idx - index as usize > 32768{
            break;
        }
        if max_len == 258{
            break;
        }
        // println!("index: {}", index);
        let mut found_idx = index as usize;
        let start_match_idx = found_idx;
        let mut temp_buffer_idx = *buffer_idx; 

        // go through the values comparing if they're right

        let key = (buffer[temp_buffer_idx-2], buffer[temp_buffer_idx-1], buffer[temp_buffer_idx]);
        temp_hash.add_trigram(key);
        // temp_hash.add_char(buffer[temp_buffer_idx]);
        
        loop {
            if temp_buffer_idx >= buffer.len() {
                break;
            }
            // No need to keep looking
            if temp_buffer_idx - *buffer_idx >= 258{
                if temp_buffer_idx - *buffer_idx > max_len {
                    curr_dist = *buffer_idx - start_match_idx;
                    max_len = temp_buffer_idx - *buffer_idx;
                }
                break 'index_loop;
            }   
            // println!("found idx {}", found_idx);
            if true_matches.dict[found_idx & LZ_DICT_SIZE_MASK] != buffer[temp_buffer_idx]{
                break;
            }             
            temp_hash.add_char(buffer[temp_buffer_idx]);

            if temp_buffer_idx >= 2 {
                if temp_buffer_idx > max_temp_buffer_idx{
                    true_matches.dict[temp_buffer_idx & LZ_DICT_SIZE_MASK] = buffer[temp_buffer_idx];
                    // checks if it has been added already 
                    if true_matches.hash[temp_hash.hash as usize] != (temp_buffer_idx - 2) & LZ_DICT_SIZE_MASK {
                        true_matches.next[(temp_buffer_idx - 2) & LZ_DICT_SIZE_MASK] = true_matches.hash[temp_hash.hash as usize];
                        true_matches.hash[temp_hash.hash as usize] = (temp_buffer_idx - 2) & LZ_DICT_SIZE_MASK;
                    }
                }
                max_temp_buffer_idx = max_temp_buffer_idx.max(temp_buffer_idx);
            }
            temp_buffer_idx += 1;
            found_idx += 1;
        }

        if temp_buffer_idx - *buffer_idx > max_len {
            curr_dist = *buffer_idx - start_match_idx;
            max_len = temp_buffer_idx - *buffer_idx;
        }
        
        let next_idx = true_matches.next[index as usize];
        index = next_idx;
    }


    let curr_option = if max_len >=3 {
        Some((curr_dist, max_len))
    } else {
        None
    };

    curr_option
}




pub fn lz77_compression_new(zodie: Vec<u8>, compressed: &mut Vec<bool>){

    let mut buffer = zodie.clone();
    buffer.insert(0, 0);
    let mut true_matches = HashBuffers::default(); //FxHashMap<(u8, u8, u8), LinkedList<usize>> = FxHashMap::default();

    if buffer.len() < 3{
        for num in buffer{
            compressed.append(&mut reverse_huffman(num));
        }
        return;
    }

    let mut curr_hash = RollingHash::new();

    // let mut dict = HashBuffers::default();

    let mut buffer_idx = 1;
    let src_buf_left = zodie.len();

    while buffer_idx < buffer.len(){
        // updates the hash

        curr_hash.add_char(buffer[buffer_idx]);
        // update the dictionary

        if buffer_idx < MAX_MATCH_LEN - 1 {
            true_matches.dict[LZ_DICT_SIZE + buffer_idx] = buffer[buffer_idx];
        }

        true_matches.dict[buffer_idx & LZ_DICT_SIZE_MASK] = buffer[buffer_idx];
        if buffer_idx >= 2 {
            // let entry = (buffer[buffer_idx-2], buffer[buffer_idx-1], buffer[buffer_idx]);
            // Add the start of the trigram into the hash table
            // curr_hash.add_trigram(entry);
            if true_matches.hash[curr_hash.hash as usize] != (buffer_idx - 2) & LZ_DICT_SIZE_MASK {
                true_matches.next[(buffer_idx - 2) & LZ_DICT_SIZE_MASK] = true_matches.hash[curr_hash.hash as usize];
                true_matches.hash[curr_hash.hash as usize] = (buffer_idx - 2) & LZ_DICT_SIZE_MASK;
            }

        }

        let matches = find_match_buffer_rolling(&buffer, &buffer_idx, &mut true_matches);
        
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
    // dbg!(true_matches);
    
}