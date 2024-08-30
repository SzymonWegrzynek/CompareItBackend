use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use base64::{engine::general_purpose, Engine};


pub fn get_stock_image(file_path: &str) -> Vec<u8> {
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut storage: Vec<u8> = Vec::new();
    reader.read_to_end(&mut storage).unwrap();
    storage
}

pub fn to_base64(image: Option<Vec<u8>>) -> String {
    match image {
        Some(data) => general_purpose::STANDARD.encode(&data),
        None => String::new()
    }
}
