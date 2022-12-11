use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[inline]
pub fn read_file_string(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    let _ = File::open(file_path)?.read_to_string(&mut buffer);
    Ok(buffer)
}

#[inline]
pub fn read_file(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer = vec![];
    let _ = File::open(file_path)?.read_to_end(&mut buffer);
    Ok(buffer)
}

#[inline]
pub fn to_u8(s: &[u8]) -> u8 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u8)
}

#[inline]
pub fn to_u32(s: &[u8]) -> u32 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u32)
}
