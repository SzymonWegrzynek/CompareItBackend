use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;


pub fn get_stock_image(file_path: &str) -> Vec<u8> {
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut storage: Vec<u8> = Vec::new();
    reader.read_to_end(&mut storage).unwrap();
    storage
}
