use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::str;

// use crate::huffman::*;
// mod huffman;

use utils::huffman::*;
// use crate::utils::get_bit;


const  COMPRESSION_METHOD_IDX : (usize, usize) = (8, 10);

struct EndCentralDirectoryRecord {
    end_central_directory_signature: [u8; 4],
    number_of_this_disk: u16,
    number_of_the_start_disk: u16,
    total_number_of_entries_on_this_disk: u16,
    total_number_of_entries: u16,
    size_of_central_directory: u32,
    offset_of_start_of_central_directory: u32,
    zip_file_comment_length: u16,
}

impl EndCentralDirectoryRecord {
    fn create_central_directory_record(central_size: u32, offset_size_central: u32) -> Self {
        EndCentralDirectoryRecord {
            end_central_directory_signature: [0x50, 0x4b, 0x05, 0x06],
            number_of_this_disk: 0,
            number_of_the_start_disk: 0,
            total_number_of_entries_on_this_disk: 1,
            total_number_of_entries: 1,
            size_of_central_directory: central_size,
            offset_of_start_of_central_directory: offset_size_central,
            zip_file_comment_length: 0,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.end_central_directory_signature);
        bytes.extend_from_slice(&self.number_of_this_disk.to_le_bytes());
        bytes.extend_from_slice(&self.number_of_the_start_disk.to_le_bytes());
        bytes.extend_from_slice(&self.total_number_of_entries_on_this_disk.to_le_bytes());
        bytes.extend_from_slice(&self.total_number_of_entries.to_le_bytes());
        bytes.extend_from_slice(&self.size_of_central_directory.to_le_bytes());
        bytes.extend_from_slice(&self.offset_of_start_of_central_directory.to_le_bytes());
        bytes.extend_from_slice(&self.zip_file_comment_length.to_le_bytes());
        bytes
    }



}

struct CentralDirectoryRecord {
    central_directory_signature: [u8; 4],
    specific_version: [u8; 1],
    made_by: [u8; 1],
    extract_version: [u8; 2],
    general_purpose_bit_flag : [u8; 2],
    compression_method: [u8; 2],
    last_mod_time: [u8; 2],
    last_mod_date: [u8; 2],
    crc_32: [u8; 4],
    compressed_file_size: [u8; 4],
    uncompressed_file_size: [u8; 4],
    file_name_length: u16,
    extra_field_length: u16,
    file_comment_length: u16,
    disk_number_start: u16,
    internal_file_attributes: u16,
    external_file_attributes: u32,
    offset_local_header: u32,
    file_name: Vec<u8>,    
}

impl CentralDirectoryRecord {
    fn create_central_directory_record(file_name: &str, file_size: u32, compressed_file_size: u32) -> Self{
        let file_name_bytes = file_name.as_bytes();
        CentralDirectoryRecord {
            central_directory_signature : [0x50, 0x4b, 0x01, 0x02],
            specific_version: [30],
            made_by: [65],
            extract_version: [20, 00],
            general_purpose_bit_flag : [0, 0],
            compression_method: [0x08, 0x00],
            last_mod_time: [0, 0],
            last_mod_date: [0, 0],
            crc_32: [0xef, 0xbe, 0xad, 0xde],
            compressed_file_size: compressed_file_size.to_le_bytes(),
            uncompressed_file_size: file_size.to_le_bytes(),
            file_name_length: file_name_bytes.len() as u16,
            extra_field_length: 0,
            file_comment_length: 0,
            disk_number_start: 0,
            internal_file_attributes: 1,
            external_file_attributes: 1,
            offset_local_header: 0,
            file_name: file_name_bytes.to_vec(),
        }
    }
    fn to_bytes (&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.central_directory_signature);
        bytes.extend_from_slice(&self.specific_version);
        bytes.extend_from_slice(&self.made_by);
        bytes.extend_from_slice(&self.extract_version);
        bytes.extend_from_slice(&self.general_purpose_bit_flag);
        bytes.extend_from_slice(&self.compression_method);
        bytes.extend_from_slice(&self.last_mod_time);
        bytes.extend_from_slice(&self.last_mod_date);
        bytes.extend_from_slice(&self.crc_32);
        bytes.extend_from_slice(&self.compressed_file_size);
        bytes.extend_from_slice(&self.uncompressed_file_size);
        bytes.extend_from_slice(&self.file_name_length.to_le_bytes());
        bytes.extend_from_slice(&self.extra_field_length.to_le_bytes());
        bytes.extend_from_slice(&self.file_comment_length.to_le_bytes());
        bytes.extend_from_slice(&self.disk_number_start.to_le_bytes());
        bytes.extend_from_slice(&self.internal_file_attributes.to_le_bytes());
        bytes.extend_from_slice(&self.external_file_attributes.to_le_bytes());
        bytes.extend_from_slice(&self.offset_local_header.to_le_bytes());
        bytes.extend_from_slice(&self.file_name);
        bytes
    }

    fn get_size(&self) -> u32{
        return self.file_name.len() as u32 + 46;
    }
}

#[derive(Debug)]
struct LocalFileRecord {
    local_file_signature: [u8; 4],
    extract_version: [u8; 2],
    general_purpose_flag: [u8; 2],
    compression_method: [u8; 2],
    last_mod_file_time: [u8; 2],
    last_mod_file_date: [u8; 2],
    crc_32: [u8; 4],

    extra_field_length : [u8; 2],

    file_name_length: u16,
    compressed_file_size : u32,
    uncompressed_file_size : u32,

    file_data: Vec<u8>, 
    file_name: Vec<u8>,
}

impl LocalFileRecord {
    fn create_local_file_record(file_name: &str) -> Self {

        let mut f = File::open(file_name).expect("couldn't open file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("couldn't read file");  

        // let file_contents = fs::read_to_string(file_name).expect("Failed to read file");

        let uncompressed_size = buffer.len();

        let file_contents = compress(buffer);

        let file_size = file_contents.len();
        // println!("file name lenth: {:?}", file_name.len());

        let file_name_bytes = file_name.as_bytes();
        LocalFileRecord {
            local_file_signature: [0x50, 0x4b, 0x03, 0x04],

            // TODO: Is this 20 int, or 20 in hex?
            extract_version: [20, 0],
            general_purpose_flag: [0, 0],
            // Compression method in little endian
            compression_method: [0x08, 0x00],
            last_mod_file_time: [0x00, 0x00],
            last_mod_file_date: [0x00, 0x00],
            crc_32: [0xef, 0xbe, 0xad, 0xde],
            file_data : file_contents,
            file_name: file_name_bytes.to_vec(), // Converted to Vec<u8>
            file_name_length: file_name.len() as u16,
            extra_field_length : [0x00, 0x00],
    
            compressed_file_size : file_size as u32,
            uncompressed_file_size : uncompressed_size as u32,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        if self.compression_method == [0x00, 0x00] {
            println!("no compression");
        }
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.local_file_signature);
        bytes.extend_from_slice(&self.extract_version);
        bytes.extend_from_slice(&self.general_purpose_flag);
        bytes.extend_from_slice(&self.compression_method);
        bytes.extend_from_slice(&self.last_mod_file_time);
        bytes.extend_from_slice(&self.last_mod_file_date);
        bytes.extend_from_slice(&self.crc_32);
        
        bytes.extend_from_slice(&self.compressed_file_size.to_le_bytes());
        bytes.extend_from_slice(&self.uncompressed_file_size.to_le_bytes());

        bytes.extend_from_slice(&self.file_name_length.to_le_bytes());
        bytes.extend_from_slice(&self.extra_field_length);
        bytes.extend_from_slice(&self.file_name);
        bytes.extend_from_slice(&self.file_data);
        bytes
    }

    fn get_size(&self) -> u32{

        return 30 + self.file_name.len() as u32 + self.file_data.len() as u32;
    }
}





fn myzip (file_name: &str , zip_name: &str) {
    
    let mut f = File::open(file_name).expect("couldn't open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("couldn't read file");


    let mut file = File::create(zip_name).expect("failed to open");
    
    // let file_size = file_contents.as_bytes().len() as u32;
    // println!("file size: {}", file_size);

    let local_record  = LocalFileRecord::create_local_file_record(file_name);
    let central_record = CentralDirectoryRecord::create_central_directory_record(file_name, buffer.len() as u32, local_record.compressed_file_size);
    let end_record = EndCentralDirectoryRecord::create_central_directory_record(central_record.get_size(), local_record.get_size());

    let mut bytes = local_record.to_bytes(); 
    bytes.extend_from_slice(&central_record.to_bytes());
    bytes.extend_from_slice(&end_record.to_bytes());

    file.write_all(&bytes).expect("couldn't write bytes");

    // println!("contents: {}", file_contents);

    // println!("contents size {}", file_contents.as_bytes().len());
}



// Code lengths are in the dynamic code length body
// HLIT - Once we have the lengths, we can decode HLIT codes with the same algorithm

// Like before, we can 
// HDIST 


// We can store the codes as a bool vec array, or numerical values by length 



fn main() {
    let args: Vec<String> = env::args().collect();
    let zip_name : &String = &args[1];
    let file_name : &String = &args[2];

    let args = dbg!(&args);

    myzip(file_name, zip_name);
}
