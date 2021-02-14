use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
mod index;

#[derive(std::clone::Clone)]
pub struct Repository {
    path: String,
}

pub enum RepoError {
    RepoAlreadyExists,
    IOError,
    RollbackError,
    IndexParsingError,
}

fn rollback(path: String) -> io::Result<()> {
    fs::remove_dir_all(path)
}

impl Repository {
    pub fn new(path: String) -> Self {
        Repository { path: path.clone() }
    }

    pub fn init(mut self) -> Result<(), RepoError> {
        let mut path = self.path + "\\.yit";
        if Path::new(&path).exists() {
            return Err(RepoError::RepoAlreadyExists);
        }
        let mut res = fs::create_dir(path.clone());
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        res = fs::create_dir(path.clone() + "\\objects");
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        res = fs::create_dir(path.clone() + "\\refs");
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        res = fs::create_dir(path.clone() + "\\refs\\heads");
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        res = fs::create_dir(path.clone() + "\\refs\\tags");
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        let file_res = File::create(path.clone() + "//HEAD");
        match file_res {
            Err(_) => Err(RepoError::IOError),
            Ok(mut file) => {
                res = file.write_all(b"ref: refs/heads/master");
                if res.is_err() {
                    res = rollback(path);
                    if res.is_err() {
                        return Err(RepoError::RollbackError);
                    }
                    return Err(RepoError::IOError);
                }
                Ok(())
            }
        }
    }

    pub fn add(self, file_path: String) -> Result<(), RepoError> {
        let res = File::create(self.path.clone() + "\\.yit\\index");
        let index_file_path = self.path.clone() + "\\.yit\\index";
        match res {
            Err(_) => Err(RepoError::IOError),
            Ok(_) => {
                let res = index::Index::new(index_file_path.clone());
                match res {
                    Err(_) => Err(RepoError::IndexParsingError),
                    Ok(index_obj) => {
                        if index_obj.clone().tracks_file(file_path.clone()) {
                            if index_obj.clone().has_different_hash(file_path.clone()) {
                                index_obj.clone().update_hash(file_path);
                            } else {
                            }
                        } else {
                            let res = index_obj.clone().add_obj(file_path, index_file_path);
                            if res.is_err() {
                                return Err(RepoError::IndexParsingError);
                            }
                        }
                        Ok(())
                    }
                }
            }
        }
    }
}
