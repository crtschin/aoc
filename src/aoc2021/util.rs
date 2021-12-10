use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[inline]
pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    let _ = File::open(file_path)?.read_to_string(&mut buffer);
    Ok(buffer)
}
