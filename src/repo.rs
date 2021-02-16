use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
mod index;
mod tree;
mod commit; 

#[derive(std::clone::Clone)]
pub struct Repository {
    path: String,
}

pub enum RepoError {
    RepoAlreadyExists,
    IOError,
    RollbackError,
    IndexParsingError,
    CommitError,
}

fn rollback(path: String) -> io::Result<()> {
    fs::remove_dir_all(path)
}

impl Repository {
    pub fn new(path: String) -> Self {
        Repository { path: path.clone()}
    }

    pub fn init(self) -> Result<(), RepoError> {
        let path = self.path + "\\.yit";
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
        let index_file_path = self.path.clone() + "\\.yit\\index";
        let full_file_path = self.path.clone() + "\\" + &file_path.clone();
        let res = index::Index::new(index_file_path.clone());
        match res {
            Err(_) => Err(RepoError::IndexParsingError),
            Ok(index_obj) => {
                if index_obj.clone().tracks_file(file_path.clone()) {
                    if index_obj.clone().has_different_hash(file_path.clone()) {
                        let res = index_obj.clone().add_obj(file_path, full_file_path, index_file_path);
                        if res.is_err() {
                            return Err(RepoError::IndexParsingError);
                        }
                    }
                } else {
                    let res = index_obj.clone().add_obj(file_path, full_file_path, index_file_path);
                    if res.is_err() {
                        return Err(RepoError::IndexParsingError);
                    }
                }
                Ok(())
            }
        }
    }

    pub fn commit(self, message: String, parents: Vec<String>) -> Result<(), RepoError>  {
        let res = File::open(self.path.clone() + "\\.yit\\index");
        let index_file_path = self.path.clone() + "\\.yit\\index";
        
        match res {
            Err(_) => Err(RepoError::IOError),
            Ok(_) => {
                let res = index::Index::new(index_file_path.clone());
                match res {
                    Err(_) => Err(RepoError::IndexParsingError),
                    Ok(index_obj) => {
                        let tr = tree::Tree::new(index_obj.index_map);
                        let (hash, file_content) = tr.hash_tree();
                        tree::Tree::write_tree(hash.clone(), file_content);
                        let res = commit::write_commit(message, parents, hash);
                        match res {
                            Err(_) => Err(RepoError::CommitError),
                            Ok(hash) => {
                                println!("{}", hash);
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    }
    
}
