use crate::file;
use crate::tree;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub enum MergeError {
    Conflict,
    DoesNotHaveOrigin,
    IOError,
}

pub fn three_fold(
    parent_index_map: HashMap<String, String>,
    branch_index_map: HashMap<String, String>,
    into_branch_index_map: HashMap<String, String>,
) -> Result<String, MergeError> {
    let mut tree = tree::Tree::new(HashMap::new());
    for (key, _) in branch_index_map.clone() {
        if !parent_index_map.contains_key(&key) {
            if into_branch_index_map.contains_key(&key) {
                return Err(MergeError::DoesNotHaveOrigin);
            }
            tree.add_blob(key.clone(), key);
        }
    }

    for (key, _) in into_branch_index_map.clone() {
        if !parent_index_map.contains_key(&key) {
            if branch_index_map.contains_key(&key) {
                return Err(MergeError::DoesNotHaveOrigin);
            }
            tree.add_blob(key.clone(), key);
        }
    }

    for (key, _) in branch_index_map.clone() {
        if into_branch_index_map.clone().contains_key(&key) {
            let new_file_content = merge_blobs(
                &parent_index_map[&key],
                &branch_index_map[&key],
                &into_branch_index_map[&key],
            )?;
            let mut file = File::create(key.clone()).unwrap();
            let res = file.write_all(new_file_content.as_bytes());
            if res.is_err() {
                return Err(MergeError::IOError);
            }
            tree.add_blob(key.clone(), key);
        }
    }
    let tree_hash = tree.hash_tree();
    Ok(tree_hash)
}

pub fn merge_blobs(
    hash_parent_blob: &str,
    hash_blob: &str,
    hash_into_blob: &str,
) -> Result<String, MergeError> {
    match file::cat_file(String::from(hash_parent_blob)) {
        Err(_) => Err(MergeError::IOError),
        Ok(parent_blob) => match file::cat_file(String::from(hash_blob)) {
            Err(_) => Err(MergeError::IOError),
            Ok(blob) => match file::cat_file(String::from(hash_into_blob)) {
                Err(_) => Err(MergeError::IOError),
                Ok(into_blob) => {
                    let parent_blob_lines: Vec<&str> = parent_blob.split('\n').collect();
                    let blob_lines: Vec<&str> = blob.split('\n').collect();
                    let into_blob_lines: Vec<&str> = into_blob.split('\n').collect();
                    let mut new_file: String = String::from("");
                    let mut cnt = 0;
                    for line in into_blob_lines {
                        if cnt >= blob_lines.len() {
                            new_file.push_str(&(String::from(line) + "\n"));
                        } else {
                            if line == blob_lines[cnt] {
                                new_file.push_str(&(String::from(line) + "\n"));
                            } else {
                                if cnt >= parent_blob_lines.len() {
                                    return Err(MergeError::Conflict);
                                } else {
                                    if blob_lines[cnt] == parent_blob_lines[cnt] {
                                        new_file.push_str(&(String::from(line) + "\n"));
                                    } else {
                                        return Err(MergeError::Conflict);
                                    }
                                }
                            }
                        }
                        cnt += 1;
                    }
                    Ok(new_file)
                }
            },
        },
    }
}
