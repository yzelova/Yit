pub use super::index::blob;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::collections::HashMap;
use std::fs::File;
use miniz_oxide::deflate::compress_to_vec;
use std::io::prelude::*;
use std::fs;

#[derive(Clone)]
pub struct Tree {
    pub subtrees: Vec<Tree>,
    pub blobs: Vec<blob::Blob>,
    pub name: String,
}

pub enum TreeError {
    IOError,
    HashError,
}

impl Tree {
    fn get_word(str: String) -> Option<(String, String)> {
        let mut word: String = String::from("");
        let mut rest = str.clone();
        if str.clone().is_empty() {
            return None;
        }
        if !rest.is_empty() && rest.chars().nth(0).unwrap() == '\\' {
            rest = rest[1..].to_string();
        }
        while !rest.is_empty() && rest.chars().nth(0).unwrap() != '\\' {
            word = (word.to_owned() + &rest.chars().nth(0).unwrap().to_string()).to_string();
            rest = rest[1..].to_string();
        }
        return Some((word.clone(), rest));
    }

    fn add_blob(&mut self, path: String, full_path: String) {
        let rest_path = path.clone();
        let res = Tree::get_word(rest_path);
        match res {
            None => (),
            Some((word, rest)) => {
                let mut subtree_option = self
                    .subtrees
                    .iter_mut()
                    .filter(|tree| (**tree).name == word);
                match subtree_option.next() {
                    None => {
                        if rest.is_empty() {
                            let blob = blob::Blob::new(full_path.clone());
                            self.blobs.push(blob.clone());
                        } else {
                            let mut subtree = Tree {
                                subtrees: Vec::new(),
                                blobs: Vec::new(),
                                name: String::from(word),
                            };
                            subtree.add_blob(rest, full_path);
                            self.subtrees.push(subtree);
                        }
                    }
                    Some(subtree_iter) => {
                        if rest.is_empty() {
                            let blob = blob::Blob::new(path.clone());
                            subtree_iter.blobs.push(blob.clone());
                        } else {
                            subtree_iter.add_blob(rest, full_path);
                        }
                    }
                }
            }
        }
    }

    fn print(self) {
        println!("tree: {}", self.name);
        print!("blobs: ");
        for blob in self.blobs {
            print!("{} ", blob.file_path);
        }
        println!();
        println!("subtrees: ");
        for tree in self.subtrees {
            tree.print();
        }
    }

    fn index_map_to_tree(index_map: HashMap<String, String>) -> Self {
        let mut tree = Tree {
            subtrees: Vec::new(),
            blobs: Vec::new(),
            name: String::from("\\"),
        };
        for (key, _) in index_map {
            tree.add_blob(key.clone(), key);
        }
        return tree;
    }

    pub fn hash_tree(self) -> (String, String) {
        let mut content = String::from("");
        //trees
        for tree in self.subtrees {
            let (hash, _) = tree.clone().hash_tree();
            content.push_str("tree ");
            content.push_str(&tree.clone().name);
            content.push_str(" ");
            content.push_str(&hash);
            content.push_str("\n");
        }

        //blobs
        for blob in self.blobs {
            let hash_res = blob.clone().hash_object(false);
            match hash_res {
                Err(_) => (),
                Ok(hash) => {
                    content.push_str("blob ");
                    content.push_str(&blob.clone().file_path);
                    content.push_str(" ");
                    content.push_str(&hash);
                    content.push_str("\n");
                }
            }
        }
        let mut hasher = Sha1::new();
        hasher.input_str(&content);
        let result = hasher.result_str();
        return (result, content);
    }

    pub fn write_tree(hash: String, content: String) -> Result<(), TreeError> {
        let dir = &hash[0..2];
        let filename = &hash[2..];
        let res = fs::create_dir(String::from(".yit\\objects\\") + dir);
        if res.is_err() {
            return Err(TreeError::IOError);
        }
        let file_res = File::create(String::from(".yit\\objects\\") + dir + "\\" + filename);
        match file_res {
            Err(_) => Err(TreeError::IOError),
            Ok(mut file) => {
                let compressed = compress_to_vec(content.as_bytes(), 0);
                let res_wr = file.write_all(&compressed);
                if res_wr.is_err() {
                    return Err(TreeError::IOError);
                }
                Ok(())
            }
        }
    }

    pub fn new(index_map: HashMap<String, String>) -> Self {
        Tree::index_map_to_tree(index_map)
    }
}
