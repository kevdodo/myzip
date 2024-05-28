use std::fs::File;
use std::io::Read;
use std::env;

fn check_same(f1: &str, f2: &str){
    let mut file1 = File::open(f1).expect("Unable to open file1.txt");
    let mut file2 = File::open(f2).expect("Unable to open file2.txt");

    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();

    file1.read_to_end(&mut buf1).expect("Unable to read file1.txt");
    file2.read_to_end(&mut buf2).expect("Unable to read file2.txt");
    

    if buf1 == buf2 {
        println!("The files are the same");
    } else {
        println!("The files are different");
    }

    for (b1, b2) in buf1.iter().take(50).zip(buf2.iter()){
        println!("yours: {:08b}, expected: {:08b}", b1, b2);
    }
    dbg!(diff_buffers(buf1, buf2));
}
fn diff_buffers(buf1: Vec<u8>, buf2: Vec<u8>) -> Vec<(usize, u8, u8)> {

    
    buf1.iter().take(50)
        .zip(buf2.iter())
        .enumerate()
        .filter_map(|(i, (&b1, &b2))| if b1 != b2 { Some((i, b1, b2)) } else { None })
        .collect()
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let file1 : &String = &args[1];
    let file2 : &String = &args[2];

    check_same(&file1, &file2);

    // b2 is bad
    // b1 is expected
    // d = 22

}