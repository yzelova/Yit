use crate::file;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use miniz_oxide::deflate::compress_to_vec;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub enum CommitError {
    IOError,
}

#[derive(std::clone::Clone)]
pub struct CommitNode {
    pub parents: Vec<CommitNode>,
    pub hash: String,
    pub tree_hash: String,
}

pub fn write_commit(
    message: String,
    parents: Vec<String>,
    tree_hash: String,
) -> Result<String, CommitError> {
    let mut content = String::from("commit\n");
    content.push_str(&tree_hash);
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
    let res = fs::create_dir_all(String::from(".yit/objects/") + dir);
    if res.is_err() {
        return Err(CommitError::IOError);
    }
    match File::create(String::from(".yit/objects/") + dir + "/" + filename) {
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

impl CommitNode {
    pub fn new(hash: String) -> Self {
        match file::cat_file(hash.clone()) {
            Err(_) => CommitNode {
                hash: String::from(""),
                tree_hash: String::from(""),
                parents: Vec::new(),
            },
            Ok(file_content) => {
                let words: Vec<&str> = file_content.split('\n').collect();
                let tree_hash = words[0];
                let mut parents: Vec<CommitNode> = Vec::new();
                for word in (words[1..]).to_vec() {
                    if word.is_empty() {
                        return CommitNode {
                            hash: hash.clone(),
                            tree_hash: String::from(tree_hash),
                            parents,
                        };
                    }
                    parents.push(CommitNode::new(String::from(word)));
                }
                CommitNode {
                    hash: String::from(""),
                    tree_hash: String::from(""),
                    parents: Vec::new(),
                }
            }
        }
    }

    pub fn is_parent(self, commit_hash: &str) -> bool {
        let mut is_parrent = false;
        for parent in self.parents {
            if parent.hash == commit_hash {
                return true;
            }
            if !is_parrent {
                is_parrent = is_parrent && parent.is_parent(commit_hash);
            }
        }
        return is_parrent;
    }

    pub fn get_most_recent_parent_commit(self, other: CommitNode) -> Option<CommitNode> {
        if other.clone().is_parent(&self.clone().hash) {
            return Some(self.clone());
        }
        let mut parent_option: Option<CommitNode> = None;
        for parent in self.parents {
            if parent_option.is_none() {
                parent_option = parent.get_most_recent_parent_commit(other.clone());
            } else {
                return parent_option;
            }
        }
        return parent_option;
    }
}
