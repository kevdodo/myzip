use std::{collections::{HashSet, HashMap}, env, fs::{self, File}, io::Write};

fn _get_temp_matches(buffer_idx: usize, buffer: &String) -> HashMap<(char, char, char), usize>{
        let mut temp_matches = HashMap::new();

        // let buff = buffer.chars();
        let buff: Vec<char> = buffer.chars().collect();
        if buffer_idx as i32 - 3 >= 0 {
            temp_matches.insert((buff[buffer_idx-3], buff[buffer_idx-2], buff[buffer_idx-1]), buffer_idx-3);
        }
        if buffer_idx as i32 - 2 >= 0 {
            temp_matches.insert((buff[buffer_idx-2], buff[buffer_idx-1], buff[buffer_idx-2]), buffer_idx - 2);
        }
        if buffer_idx > 0{
            temp_matches.insert((buff[buffer_idx-1], buff[buffer_idx-1], buff[buffer_idx-1]), buffer_idx-1);
        }
        temp_matches
    }

// Extension: Do a binary search algorithm to find index faster
fn find_match(buffer: &String, buffer_idx: &usize, true_matches: &mut HashMap<(char, char, char), Vec<usize>>) -> Option<(usize, usize)>{
    /*
    
    Curr3 is the prev 3 bytes 
    Next 3 is the look up of the next current byte + 2 bytes

    */
    // let mut curr3 = (buffer[*buffer_idx], buffer[*buffer_idx - 1], buffer[*buffer_idx -2]);

    let start_buffer = *buffer_idx;

    let mut max_len = 0;
    let mut curr_dist = 0;

    let temp_matches = _get_temp_matches(*buffer_idx, buffer);

    // let temp_matches = dbg!(temp_matches);

    let buff: Vec<char> = buffer.chars().collect();   
    let mut curr_option = None; 

    if *buffer_idx + 2 >= buff.len(){
        // println!("max length: {}", max_len);
        if max_len != 0{
            curr_option = Some((curr_dist, max_len));
        } else {
            curr_option = None;
        }
        return curr_option;
    }

    let next_3_bytes = (buff[*buffer_idx], buff[*buffer_idx + 1], buff[*buffer_idx + 2]);

    // let temp_matches = dbg!(temp_matches);
    
    if let Some(index) = temp_matches.get(&next_3_bytes){
        // this wont work, need to find the right index
        let mut found_idx = *index;
        let start_match_idx = found_idx;
        let mut temp_buffer_idx = *buffer_idx + 2; 

        while true {
            if temp_buffer_idx >= 3 {
                let val = true_matches.entry((buff[temp_buffer_idx-3], buff[temp_buffer_idx-2], buff[temp_buffer_idx-1])).or_insert(Vec::new());
                if !val.contains(&(temp_buffer_idx-2)){
                    val.insert(0, temp_buffer_idx-3);
                    let val = val;
                    // println!("it happendefasdf");
                }
            }
            if temp_buffer_idx < buffer.len() {
                let next_el = buff[found_idx];
                if next_el != buff[temp_buffer_idx] || (temp_buffer_idx - *buffer_idx >= 258){
                    break;
                }
                temp_buffer_idx += 1;
                found_idx += 1;
            } else {
                break;
            }
            // updates the max length
            if (temp_buffer_idx - *buffer_idx >= max_len) {
                curr_dist = *buffer_idx - start_match_idx;
                max_len = temp_buffer_idx - *buffer_idx;
            }
        }
    }

    let true_matches = true_matches;

    // let next_3_bytes = dbg!(next_3_bytes);
    if let Some(indices) = true_matches.get(&next_3_bytes){
        let indices = indices.clone();

        for index in indices{
            if *buffer_idx <= index || buffer_idx - index > 32768{
                // println!("yuhhhh");
                continue;
            }
            // println!("index: {}", index);
            let mut found_idx = index;
            let start_match_idx = found_idx;
            let mut temp_buffer_idx = *buffer_idx; 
            if max_len == 258{
                break;
            }
            while true {
                if temp_buffer_idx >= buffer.len() {
                    break;
                }
                if temp_buffer_idx >= 2 {
                    let val = true_matches.entry((buff[temp_buffer_idx-2], buff[temp_buffer_idx-1], buff[temp_buffer_idx])).or_insert(Vec::new());
                    if !val.contains(&(temp_buffer_idx-2)){
                        val.insert(0, temp_buffer_idx-2);
                    }
                }
                if buff[found_idx] != buff[temp_buffer_idx] || (temp_buffer_idx - *buffer_idx >= 258){
                    // println!("buff found idx {}", found_idx);
                    // println!("buff temp idx {}", temp_buffer_idx);
                    break;
                }

                // println!("next_el {} | idx {}, temp_buff {} | idx {}", next_el, found_idx, buff[temp_buffer_idx], temp_buffer_idx);

                // need to update the hashmap again

                temp_buffer_idx += 1;
                found_idx += 1;
            }
            // println!("curr_dist {}, best distance: {}", *buffer_idx - start_match_idx, curr_dist);
            if temp_buffer_idx - *buffer_idx > max_len {
                // println!("temp buff idx {}, buff idx {}", temp_buffer_idx, buffer_idx);
                curr_dist = *buffer_idx - start_match_idx;
                max_len = temp_buffer_idx - *buffer_idx;
            }
        }
    }
    if max_len != 0{
        curr_option = Some((curr_dist, max_len));
    } else {
        curr_option = None;
    }

    curr_option
}



pub fn lz77_compression(buffer: &String) -> String{

    let mut compressed: Vec<char> = Vec::new();

    // hashmap of the bytes to the previous found indices
    let mut true_matches: HashMap<(char, char, char), Vec<usize>> = HashMap::new();

    if buffer.len() < 3{
        return buffer.to_string();
    }
    let buff: Vec<char> = buffer.chars().collect();

    let mut buffer_idx = 0;

    while buffer_idx < buff.len(){

        if buffer_idx >= 2 {
            let val = true_matches.entry((buff[buffer_idx-2], buff[buffer_idx-1], buff[buffer_idx])).or_insert(Vec::new());
            
            if !val.contains(&(buffer_idx-2)){
                val.insert(0, buffer_idx-2);
            }
        }

        let matches = find_match(buffer, &buffer_idx, &mut true_matches);
        match matches{
            Some(current_match_val) =>{
                let (distance, length) = current_match_val;
                compressed.push('<');
                let l = length.to_string();
                for c in l.chars(){
                    compressed.push(c);
                }
                compressed.push(',');
                let d = distance.to_string();
                for c in d.chars(){
                    compressed.push(c);
                }
                compressed.push('>');
                buffer_idx += length;

                // output the current match
            },
            None =>{
                // output match
                let curr_byte = buff[buffer_idx];
                compressed.push(curr_byte);
                buffer_idx += 1;
            }
        }
    }
    
    compressed.into_iter().collect()
}


fn main(){

    let args: Vec<String> = env::args().collect();
    let zip_name : &String = &args[1];
    let str_buffer = fs::read_to_string(zip_name).expect("not a valid file thing");

    println!("string len{}", str_buffer.len());
    
    // dbg!(find_match(&test_buff, &mut 4, &mut HashMap::new()));

    let mut new_file = File::create(zip_name.to_owned() + ".lz77").expect("Couldn't make file dude....");

    let data = dbg!(lz77_compression(&str_buffer));
    new_file.write_all(data.as_bytes()).expect("Unable to write data");
}