use crypto::digest::Digest;
use crypto::sha1::Sha1;
use miniz_oxide::deflate::compress_to_vec;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub enum CommitError {
    IOError,
}

pub fn write_commit(
    message: String,
    parents: Vec<String>,
    tree_hash: String,
) -> Result<String, CommitError> {
    let mut content = tree_hash;
    content.push_str("\n");
    for parent in parents {
        content.push_str(&parent);
        content.push_str("\n");
    }
    content.push_str("\n");
    content.push_str(&message);
    let mut hasher = Sha1::new();
    hasher.input_str(&content);
    let hash = hasher.result_str();

    let dir = &hash[0..2];
    let filename = &hash[2..];
    let res = fs::create_dir(String::from(".yit\\objects\\") + dir);
    if res.is_err() {
        return Err(CommitError::IOError);
    }
    let file_res = File::create(String::from(".yit\\objects\\") + dir + "\\" + filename);
    match file_res {
        Err(_) => Err(CommitError::IOError),
        Ok(mut file) => {
            let compressed = compress_to_vec(content.as_bytes(), 0);
            let res_wr = file.write_all(&compressed);
            if res_wr.is_err() {
                return Err(CommitError::IOError);
            }
            Ok(hash)
        }
    }
}
