use std::fs::File;
use std::io::prelude::*;

pub enum BranchError {
    IOError,
}

pub fn get_commit(branch_name: String) -> Result<String, BranchError> {
    match File::open(String::from(".yit\\refs\\heads\\") + &branch_name) {
        Err(_) => Err(BranchError::IOError),
        Ok(mut file) => {
            let mut content = String::from("");
            match file.read_to_string(&mut content) {
                Err(_) => Err(BranchError::IOError),
                Ok(_) => {
                    let words: Vec<&str> = content.split("\n").collect();
                    Ok(String::from(words[1]))
                }
            }
        }
    }
}

pub fn set_last_commit(branch_name: String, commit_hash: String) -> Result<(), BranchError> {
    let res = File::create(String::from(".yit\\refs\\heads\\") + &branch_name);
    match res {
        Err(_) => Err(BranchError::IOError),
        Ok(mut file) => {
            let content = String::from("ref\n") + &commit_hash;
            match file.write_all(content.as_bytes()) {
                Err(_) => Err(BranchError::IOError),
                Ok(_) => Ok(()),
            }
        }
    }
}
