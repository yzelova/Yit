use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use miniz_oxide::deflate::compress_to_vec;

pub enum RepositoryError {
    IOError,
    RepoDoesNotExist
}

pub fn write_object(hash: String, content: String) -> Result<(), RepositoryError> {
    if !Path::new(".yit").exists() {
        return Err(RepositoryError::RepoDoesNotExist);
    }
    let dir = &hash[0..2];
    let filename = &hash[2..];
    let res = fs::create_dir(String::from(".yit\\objects\\") + dir);
    if res.is_err() {
        return Err(RepositoryError::IOError);
    }
    let file_res = File::create(String::from(".yit\\objects\\") + dir + "\\" + filename);
    match file_res {
        Err(_) => Err(RepositoryError::IOError),
        Ok(mut file) => {
            let compressed = compress_to_vec(content.as_bytes(), 0);
            let res_wr = file.write_all(&compressed);
            if res_wr.is_err() {
                return Err(RepositoryError::IOError);
            }
            Ok(())
        }
    }
}

pub enum HashError {
    IOError,
}


#[derive(Clone)]
pub struct Blob {
    pub file_path: String,
}

impl Blob {
    pub fn new(file_path: String) -> Self {
        return Blob {
            file_path: file_path.clone(),
        };
    }

    pub fn hash_object(self, write: bool) -> Result<String, HashError> {
        let open_file_res = File::open(self.file_path.clone());
        match open_file_res {
            Err(_) => {
                Err(HashError::IOError)
            },
            Ok(mut file) => {
                let mut contents = String::new();
                let read_res = file.read_to_string(&mut contents);
                match read_res {
                    Err(_) => Err(HashError::IOError),
                    Ok(_) => {
                        let metadata_res = fs::metadata(self.file_path.clone());
                        match metadata_res {
                            Err(_) => Err(HashError::IOError),
                            Ok(metadata) => {
                                let filesize = metadata.len();
                                let to_hash = String::from("blob ").clone()
                                    + &filesize.to_string()
                                    + "\0"
                                    + &contents;
                                let mut hasher = Sha1::new();
                                hasher.input_str(&to_hash);
                                let result = hasher.result_str();
                                if write {
                                    let res = write_object(result.clone(), contents);
                                    if res.is_err() {
                                        return Err(HashError::IOError);
                                    }
                                }
                                return Ok(result);
                            }
                        }
                    }
                }
            }
        }
    }
}
