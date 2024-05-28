mod old_huffman;


mod tests{
    use std::vec;
    use utils::*;

    use super::*;
    use old_huffman::*;
    #[test]
    fn test_huffman_num(){
        let num = 255;
        dbg!(reverse_huffman(num));

        let num = 145;
        dbg!(reverse_huffman(num));

        let num = 0;
        dbg!(reverse_huffman(num));

        let num = 95;
        dbg!(reverse_huffman(num));
    }
    #[test]
    fn test_huffman_decode(){
        let bruh = "asdf";
        get_huffman(&bruh.to_string());
    }

    #[test]
    fn test_huffman_decode_file(){
        get_huffman(&r"testdata\inflate\fixed-huffman-literals-expected".to_string());

    }
    
}

fn main(){

}