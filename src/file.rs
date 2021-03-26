use miniz_oxide::inflate::decompress_to_vec;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str;

pub enum ParseError {
    IOError,
}

pub fn cat_file(hash: String) -> Result<String, ParseError> {
    let dir = &hash[0..2];
    let filename = &hash[2..];
    match File::open(String::from(".yit/objects/") + dir + "/" + filename) {
        Err(_) => Err(ParseError::IOError),
        Ok(mut file) => {
            let metadata = fs::metadata(&(String::from(".yit/objects/") + dir + "/" + filename))
                .expect("unable to read metadata");
            let mut buffer = vec![0; metadata.len() as usize];
            match file.read(&mut buffer) {
                Err(_) => Err(ParseError::IOError),
                Ok(_) => match decompress_to_vec(&buffer) {
                    Err(_) => Err(ParseError::IOError),
                    Ok(decompressed) => match str::from_utf8(&decompressed) {
                        Err(_) => Err(ParseError::IOError),
                        Ok(converted) => {
                            let converted_string = String::from(converted);
                            let lines: Vec<&str> = converted_string.split('\n').collect();
                            let content = &lines[1..];
                            let joined = content.join("\n");
                            Ok(joined)
                        }
                    },
                },
            }
        }
    }
}
