use crate::blob;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(std::clone::Clone)]
pub struct Index {
    pub index_map: HashMap<String, String>,
    pub file_path: String,
}

pub enum IndexError {
    IOError,
}

impl Index {
    pub fn new(file_path: String) -> Result<Self, IndexError> {
        let mut res = File::open(file_path.clone());
        if res.is_err() {
            let res_cr = File::create(file_path.clone());
            if res_cr.is_err() {
                return Err(IndexError::IOError);
            }
            res = File::open(file_path.clone())
        }
        match res {
            Err(_) => Err(IndexError::IOError),
            Ok(mut file) => {
                let mut contents = String::new();
                let read_res = file.read_to_string(&mut contents);
                if read_res.is_err() {
                    return Err(IndexError::IOError);
                }
                let mut words = contents.split_whitespace().peekable();
                let mut index_map = HashMap::new();
                while words.peek().is_some() {
                    let chunk: Vec<&str> = words.by_ref().take(2).collect();
                    index_map.insert(String::from(chunk[0]), String::from(chunk[1]));
                }
                Ok(Index {
                    index_map,
                    file_path: file_path.clone(),
                })
            }
        }
    }

    pub fn tracks_file(self, path: String) -> bool {
        return self.index_map.contains_key(&path);
    }

    pub fn has_different_hash(self, path: String) -> bool {
        let blob = blob::Blob::new(path.clone());
        match blob.hash_object(false) {
            Err(_) => false,
            Ok(hash) => {
                return self.index_map[&path] != hash;
            }
        }
    }

    pub fn add_obj(
        mut self,
        path: String,
        full_path: String,
        index_path: String,
    ) -> Result<(), IndexError> {
        let blob = blob::Blob::new(full_path.clone());
        match blob.hash_object(true) {
            Err(_) => Err(IndexError::IOError),
            Ok(hash) => {
                self.index_map.insert(path.clone(), hash);
                match File::create(index_path) {
                    Err(_) => Err(IndexError::IOError),
                    Ok(mut file) => match file.set_len(0) {
                        Err(_) => Err(IndexError::IOError),
                        Ok(_) => {
                            for (key, val) in self.index_map {
                                let line = key + " " + &val + "\n";
                                let res = file.write_all(line.as_bytes());
                                if res.is_err() {
                                    return Err(IndexError::IOError);
                                }
                            }
                            Ok(())
                        }
                    },
                }
            }
        }
    }
}
