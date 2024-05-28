mod dynamic;


#[cfg(test)]
mod tests {
    use std::{convert, vec};
    use utils::*;

    use super::*;
    use dynamic::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_get_num(){
        let bits = [false, false, false, false, false, false, false, true];
        assert_eq!(get_num(&bits.to_vec()), 1);

        let bits = [false, false, false, true, false, true, true, true];
        assert_eq!(get_num(&bits.to_vec()), 23);

        let bits = [true, true, true, true, true, true, true, true];
        assert_eq!(get_num(&bits.to_vec()), 255);
    }


    #[test]
    // fn test_len(){
    //     let len_buffer = [0b11000000, 0b11000110];
    //     assert_eq!(get_n_bits_regular(&len_buffer, 0, 9).len(), 9);
    //     assert_eq!(get_n_bits_regular(&len_buffer, 0, 8).len(), 8);
    //     assert_eq!(get_n_bits_regular(&len_buffer, 0, 7).len(), 7);
    // } 

    #[test]
    fn test_num_from_buff(){
        let buff = [0b10010001];

        let a = get_num_buffer(&buff, &mut 0);
        match a {
            AsciiNum:: Ascii(value) => {
                assert_eq!(value, 97);
            }
            AsciiNum:: LenDist(_value) => {
                assert!(false);
            }
        }

        let buff = [0b00000100, 0b0000];
        let mut idx = 0;

        let a = get_num_buffer(&buff, &mut idx);
        match a {
            AsciiNum:: Ascii(_value) => {
                assert!(false);
            }
            AsciiNum:: LenDist(value) => {
                assert_eq!(value, (4, 1));
            }
        }
        // assert_eq!(get_num_buffer(&buff, 8), 228);
    }
    

    #[test]
    fn test_convert_len(){
        assert_eq!(get_lower_bound(4, 277), 67);
        assert_eq!(get_lower_bound(4, 279), 99);
        assert_eq!(get_lower_bound(4, 279), 99);
        assert_eq!(get_lower_bound(5, 284), 227);
        assert_eq!(get_lower_bound(2, 269), 19);
        assert_eq!(get_lower_bound(1, 265), 11);
        assert_eq!(get_lower_bound(4, 279), 99);
    }

    #[test]
    fn test_decode_fixed_huffman_deflate(){
        let mut idx = 3;

        let bits:[u8; 5] =  [0b11010010, 0b00110010, 0b00100000, 0b10000000, 0b00000000];
        assert_eq!(decode_deflate(&bits, &mut idx), vec![97, 97, 97, 97, 97, 97]);

        let bits = [0b11010010, 0b00110010, 0b01010010, 0b01110010, 0b10010010, 0b00100001, 0b01000111, 0b00101010, 0b01110100, 0b00000000];
        
        idx = 3;

        dbg!(decode_deflate(&bits, &mut idx));

    }
    #[test]
    fn test_decode_fixed_huffman_deflate2(){
        let mut idx = 3;

        let bits:[u8; 5] =  [0b11010010, 0b00110010, 0b00100000, 0b10000000, 0b00000000];
        assert_eq!(decode_deflate(&bits, &mut idx), vec![97, 97, 97, 97, 97, 97]);

        let bits = [0b11010010, 0b00110010, 0b01010010, 0b01110010, 0b10010010, 0b00100001, 0b01000111, 0b00101010, 0b01110100, 0b00010100 , 0b01110000, 0b00000000, 0b00000000];
        
        idx = 3;
        // a, b, c, d, a, b, c, d, a, b, c, d, e, \n, c, d, e, \n, c, d, e, \n, c, d, e, 

        dbg!(decode_deflate(&bits, &mut idx));

    }
    #[test]
    fn test_get_codebook(){
        // uses the empty one
        let alphabet : [u16; 18] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1];

        let code_lengths_stream: [u8; 7] = [0b00000010, 0b00100000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00001000];
        
        let mut code_lengths : [u16; 18] = [200; 18];
        
        for i in 0..alphabet.len(){
            let num = get_num_reverse(&get_n_bits_reverse (&code_lengths_stream, i*3, 3));
            code_lengths[i] = num;
        }
        
        let code_lengths = dbg!(code_lengths).to_vec();

        // dbg!(convert_code_lengths(&code_lengths, &alphabet));
        let mat = convert_code_lengths_matrix(&code_lengths, alphabet.to_vec());

        let row_start = 0;
        let row_end = 3;
        let col_start = 0;
        let col_end = 5;

        for row in row_start..row_end {
            for col in col_start..col_end {
                print!("{} ", mat[row][col]);
            }
            println!();
        }

        // using cool array like structure now, can't use test

        // let test_lengths = [3, 3, 3, 3, 3, 2, 4, 4];
        // let test_alphabet = ['A' as u16, 'B' as u16, 'C' as u16, 'D' as u16, 'E' as u16, 'F' as u16, 'G' as u16, 'H' as u16];

        // assert_eq!(convert_code_lengths(&test_lengths, &test_alphabet).get(&('A' as u16)), Some(&(3 as u16, 2 as usize)));
    }
    #[test]
    fn test_decoding_dynamic_one_distance(){

        let block_header = [0b10000000, 0b00011100];
        let mut idx = 0; 
        let hlit = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 5));
        idx += 5;
        dbg!(get_n_bits_reverse(&block_header, idx, 5));

        let hdist = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 5)) +1;
        idx += 5;


        let hclen = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 4)) + 4;

        assert_eq!(hclen, 18);
        assert_eq!(257 + hlit, 258);
        assert_eq!(hdist, 1);

        // 18 codes total
        // let code_length_code: [u8; 7] = [0b00000001, 0b00100000, 0b00000000, 0b00000000, 0b00000000, 0b00000010, 255-0b000010];
        // let codes: [u8; 4] = [0b00101111, 0b11111111, 0b00101101, 255-0b1001];
    }

    #[test]
    fn test_dynamic_huffman_empty_from_file(){
        let mut f = File::open("dynamic-huffman-empty.deflate").expect("couldn't open file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("couldn't read file");
        let block_header: Vec<u8> = buffer.iter().map(|&x| x.reverse_bits()).collect();
    
        let block_header = dbg!(block_header);

        let mut idx = 0; 
        let bfinal = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 1));
        idx += 1;
        let btype = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 2));
        idx += 2;
        let hlit = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 5));
        idx += 5;
        let hdist = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 5));
        idx += 5;

        // dbg!(get_n_bits_reverse(&block_header, idx, 4));

        let hclen = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 4));
        idx += 4;
        assert_eq!(bfinal, 1);
        assert_eq!(btype, 2);

        assert_eq!(hclen + 4, 19);
        assert_eq!(257 + hlit, 257);
        assert_eq!(hdist + 1, 2);

        let code_lengths = dbg!(get_code_length_code_matrix(&block_header, &mut idx, hclen));

        let alphabet : [u16; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

        let mat = convert_code_lengths_matrix(&code_lengths, alphabet.to_vec());
        
        let row_start = 0;
        let row_end = 3;
        let col_start = 0;
        let col_end = 5;

        for row in row_start..mat.len() {
            for col in col_start..row_end {
                print!("{} ", mat[row][col]);
            }
            println!();
        }

        // The codes: 

        // 0 -> 00
        // 2 -> 10
        // 18 -> 11
        // 01 -> 01
        
        let lit_codes = get_codes_lit_dist(&mat, &block_header, &mut idx, hlit as usize + 257);

        println!("lit_codes matrix:");
        for row in row_start..lit_codes.len() {
            for col in col_start..col_end {
                print!("{} ", lit_codes[row][col]);
            }
            println!();
        }
        // dbg!(lit_codes);

        let dist_codes = get_codes_lit_dist(&mat, &block_header, &mut idx, hdist as usize + 1); 
        println!("dist codes matrix: ");
        for row in row_start..dist_codes.len() {
            for col in col_start..col_end {
                print!("{} ", dist_codes[row][col]);
            }
            println!();
        }        // dbg!(dist_codes);

        let decoded = dbg!(decode_dynamic(lit_codes, dist_codes, &block_header, &mut idx));


        assert_eq!(decoded.len(), 0);
    }


    #[test]
    fn test_dynamic_huffman_empty_no_distance_from_file(){
        let mut f = File::open(r"testdata\inflate\dynamic-huffman-empty-no-distance-code.deflate").expect("couldn't open file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("couldn't read file");
        let block_header: Vec<u8> = buffer.iter().map(|&x| x.reverse_bits()).collect();
    
        let block_header = dbg!(block_header);

        let mut idx = 0; 
        let bfinal = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 1));
        idx += 1;
        let btype = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 2));
        idx += 2;
        let hlit = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 5));
        idx += 5;
        let hdist = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 5));
        idx += 5;

        // dbg!(get_n_bits_reverse(&block_header, idx, 4));

        let hclen = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 4));
        idx += 4;
        assert_eq!(bfinal, 1);
        assert_eq!(btype, 2);

        assert_eq!(hclen + 4, 18);
        assert_eq!(257 + hlit, 257);
        assert_eq!(hdist + 1, 1);

        let code_lengths = dbg!(get_code_length_code_matrix(&block_header, &mut idx, hclen));

        let alphabet : [u16; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

        let mat = convert_code_lengths_matrix(&code_lengths, alphabet.to_vec());
        
        let row_start = 0;
        let row_end = 3;
        let col_start = 0;
        let col_end = 5;

        for row in row_start..mat.len() {
            for col in col_start..row_end {
                print!("{} ", mat[row][col]);
            }
            println!();
        }

        // The codes: 

        // 0 -> 00
        // 2 -> 10
        // 18 -> 11
        // 01 -> 01
        
        let lit_codes = get_codes_lit_dist(&mat, &block_header, &mut idx, hlit as usize + 257);

        println!("lit_codes matrix:");
        for row in row_start..lit_codes.len() {
            for col in col_start..col_end {
                print!("{} ", lit_codes[row][col]);
            }
            println!();
        }
        // dbg!(lit_codes);


        println!("whattttt");
        dbg!(get_n_bits_reverse(&block_header, idx, 5));


        let dist_codes = get_codes_lit_dist(&mat, &block_header, &mut idx, hdist as usize + 1); 
        println!("dist codes matrix: ");
        for row in row_start..dist_codes.len() {
            for col in col_start..col_end {
                print!("{} ", dist_codes[row][col]);
            }
            println!();
        }        // dbg!(dist_codes);

        let decoded = dbg!(decode_dynamic(lit_codes, dist_codes, &block_header, &mut idx));


        assert_eq!(decoded.len(), 0);
    }

    #[test]
    fn test_decoding_dynamic_one_distance_test_from_file(){

        let mut f = File::open("dynamic-huffman-one-distance-code.deflate").expect("couldn't open file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("couldn't read file");
        let block_header: Vec<u8> = buffer.iter().map(|&x| x.reverse_bits()).collect();
    
        let block_header = dbg!(block_header);

        let mut idx = 0; 
        let bfinal = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 1));
        idx += 1;
        let btype = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 2));
        idx += 2;
        let hlit = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 5));
        idx += 5;
        let hdist = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 5));
        idx += 5;

        // dbg!(get_n_bits_reverse(&block_header, idx, 4));

        let hclen = get_num_reverse(&get_n_bits_reverse(&block_header, idx, 4));
        idx += 4;
        assert_eq!(bfinal, 1);
        assert_eq!(btype, 2);

        assert_eq!(hclen + 4, 18);
        assert_eq!(257 + hlit, 258);
        assert_eq!(hdist + 1, 1);
        let code_lengths = dbg!(get_code_length_code_matrix(&block_header, &mut idx, hclen));
        
        let alphabet : [u16; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

        let mat = convert_code_lengths_matrix(&code_lengths, alphabet.to_vec());
        
        let row_start = 0;
        let row_end = 3;
        let col_start = 0;
        let col_end = 5;

        for row in row_start..row_end {
            for col in col_start..col_end {
                print!("{} ", mat[row][col]);
            }
            println!();
        }

        // The codes: 

        // 0 -> 00
        // 2 -> 10
        // 18 -> 11
        // 01 -> 01
        
        let lit_codes = get_codes_lit_dist(&mat, &block_header, &mut idx, hlit as usize + 257);

        println!("lit_codes matrix:");
        for row in row_start..row_end {
            for col in col_start..col_end {
                print!("{} ", lit_codes[row][col]);
            }
            println!();
        }
        // dbg!(lit_codes);

        let dist_codes = get_codes_lit_dist(&mat, &block_header, &mut idx, hdist as usize + 1); 
        for row in row_start..2 {
            for col in col_start..col_end {
                print!("{} ", dist_codes[row][col]);
            }
            println!();
        }        // dbg!(dist_codes);

        let decoded = dbg!(decode_dynamic(lit_codes, dist_codes, &block_header, &mut idx));


        assert_eq!(vec![01, 01, 01, 01], decoded);
        // get_code_length_code_matrix()

    }

    #[test]
    fn check_len_convert(){
        let l = dbg!(convert_length(258));

        println!("lenghts fa {}", l.len());
    }

    #[test]
    fn check_dist_convert(){
        let l = dbg!(convert_dist(3));

        println!("dist{}", l.len());
    }
    // 00010010
    
}

fn main(){
    
}