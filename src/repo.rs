use crate::branch;
use crate::commit;
use crate::diff;
use crate::index;
use crate::merge;
use crate::tree;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(std::clone::Clone)]
pub struct Repository {}

pub enum RepoError {
    RepoAlreadyExists,
    IOError,
    RollbackError,
    IndexParsingError,
    CommitError,
    MergeError,
    CheckoutError,
}

fn rollback(path: String) -> io::Result<()> {
    fs::remove_dir_all(path)
}

impl Repository {
    pub fn new() -> Self {
        Repository {}
    }

    pub fn init(self) -> Result<(), RepoError> {
        let path = String::from(".yit");
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
        res = fs::create_dir(path.clone() + "/objects");
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        res = fs::create_dir(path.clone() + "/refs");
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        res = fs::create_dir(path.clone() + "/refs/heads");
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        res = fs::create_dir(path.clone() + "/refs/tags");
        if res.is_err() {
            res = rollback(path);
            if res.is_err() {
                return Err(RepoError::RollbackError);
            }
            return Err(RepoError::IOError);
        }
        let file_res = File::create(path.clone() + "/HEAD");
        match file_res {
            Err(_) => Err(RepoError::IOError),
            Ok(mut file) => {
                res = file.write_all(b".yit/refs/heads/master");
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
        let index_file_path = String::from(".yit/index");
        let full_file_path = file_path.clone();
        match index::Index::new(index_file_path.clone()) {
            Err(_) => Err(RepoError::IndexParsingError),
            Ok(index_obj) => {
                if index_obj.clone().tracks_file(file_path.clone()) {
                    if index_obj.clone().has_different_hash(file_path.clone()) {
                        let res =
                            index_obj
                                .clone()
                                .add_obj(file_path, full_file_path, index_file_path);
                        if res.is_err() {
                            return Err(RepoError::IndexParsingError);
                        }
                    }
                } else {
                    let res = index_obj
                        .clone()
                        .add_obj(file_path, full_file_path, index_file_path);
                    if res.is_err() {
                        return Err(RepoError::IndexParsingError);
                    }
                }
                Ok(())
            }
        }
    }

    pub fn commit(self, message: String) -> Result<(), RepoError> {
        let res;
        if !Path::new(".yit/index").exists() {
            res = File::create(".yit/index");
        } else {
            res = File::open(".yit/index");
        }
        match Repository::get_current_head_last_commit() {
            Err(_) => return Err(RepoError::CommitError),
            Ok(last_commit) => {
                let index_file_path = String::from(".yit/index");
                match res {
                    Err(_) => Err(RepoError::IOError),
                    Ok(_) => match index::Index::new(index_file_path.clone()) {
                        Err(_) => Err(RepoError::IndexParsingError),
                        Ok(index_obj) => match Repository::get_current_head() {
                            Err(_) => Err(RepoError::IOError),
                            Ok(head) => {
                                let mut index_map: HashMap<String, String> = HashMap::new();
                                if Path::new(&head).exists() {
                                    let commit_tree = commit::CommitNode::new(last_commit);
                                    index_map =
                                        tree::Tree::tree_to_index_map(commit_tree.tree_hash);
                                }
                                index_map.extend(index_obj.index_map);
                                let tr = tree::Tree::new(index_map);
                                let hash = tr.hash_tree();
                                match Repository::get_current_head_last_commit() {
                                    Err(_) => Err(RepoError::CommitError),
                                    Ok(parent_commit) => {
                                        match commit::write_commit(
                                            message,
                                            vec![parent_commit],
                                            hash,
                                        ) {
                                            Err(_) => Err(RepoError::CommitError),
                                            Ok(hash) => match File::create(head) {
                                                Err(_) => Err(RepoError::IOError),
                                                Ok(mut file) => {
                                                    let content = String::from("ref\n") + &hash;
                                                    match file.write_all(content.as_bytes()) {
                                                        Err(_) => Err(RepoError::CommitError),
                                                        Ok(_) => {
                                                            match fs::remove_file(".yit/index") {
                                                                Err(_) => {
                                                                    Err(RepoError::CommitError)
                                                                }
                                                                Ok(_) => Ok(()),
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        },
                    },
                }
            }
        }
    }

    fn change_head_last_commit(path: String) -> Result<(), RepoError> {
        match File::create(path) {
            Err(_) => Err(RepoError::IOError),
            Ok(mut file) => match Repository::get_current_head_last_commit() {
                Err(_) => Err(RepoError::IOError),
                Ok(commit_hash) => {
                    let content = String::from("ref\n") + &commit_hash;
                    let res = file.write_all(content.as_bytes());
                    if res.is_err() {
                        return Err(RepoError::IOError);
                    }
                    Ok(())
                }
            },
        }
    }

    fn get_current_head() -> Result<String, RepoError> {
        match File::open(String::from(".yit/HEAD")) {
            Err(_) => Err(RepoError::IOError),
            Ok(mut head_file) => {
                let mut contents = String::new();
                match head_file.read_to_string(&mut contents) {
                    Err(_) => Err(RepoError::IOError),
                    Ok(_) => Ok(contents),
                }
            }
        }
    }

    fn get_current_head_last_commit() -> Result<String, RepoError> {
        match File::open(String::from(".yit/HEAD")) {
            Err(_) => Err(RepoError::IOError),
            Ok(mut head_file) => {
                let mut contents = String::new();
                let read_res = head_file.read_to_string(&mut contents);
                if read_res.is_err() {
                    return Err(RepoError::IOError);
                }
                match File::open(contents.clone()) {
                    Err(_) => Ok(String::from("")),
                    Ok(mut ref_file) => {
                        let mut contents = String::new();
                        let read_res = ref_file.read_to_string(&mut contents);
                        if read_res.is_err() {
                            return Err(RepoError::IOError);
                        }
                        let words: Vec<&str> = contents.split('\n').collect();
                        Ok(String::from(words[1]))
                    }
                }
            }
        }
    }

    pub fn checkout(self, branch_name: String) -> Result<(), RepoError> {
        if !Path::new(&(String::from(".yit/refs/heads/") + &branch_name)).exists() {
            let res = Repository::change_head_last_commit(
                String::from(".yit/refs/heads/") + &branch_name,
            );
            if res.is_err() {
                return Err(RepoError::CheckoutError);
            }
        }
        let head_file_res = File::create(String::from(".yit/HEAD"));
        match head_file_res {
            Err(_) => Err(RepoError::IOError),
            Ok(mut head_file) => {
                let res = head_file
                    .write_all((String::from(".yit/refs/heads/") + &branch_name).as_bytes());
                if res.is_err() {
                    return Err(RepoError::IOError);
                }
                if Path::new(".yit/index").exists() {
                    let res = std::fs::remove_file(".yit/index");
                    if res.is_err() {
                        return Err(RepoError::IOError);
                    }
                }
                match branch::get_commit(branch_name) {
                    Err(_) => Err(RepoError::CheckoutError),
                    Ok(commit) => {
                        let commit_tree = commit::CommitNode::new(commit.clone());
                        let tree_index_map = tree::Tree::tree_to_index_map(commit_tree.tree_hash);
                        tree::Tree::load_index_map(tree_index_map);
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn merge(self, branch: String, into_branch: String) -> Result<(), RepoError> {
        match branch::get_commit(branch.clone()) {
            Err(_) => Err(RepoError::MergeError),
            Ok(commit) => {
                let commit_tree = commit::CommitNode::new(commit.clone());
                match branch::get_commit(into_branch.clone()) {
                    Err(_) => Err(RepoError::MergeError),
                    Ok(into_commit) => {
                        let into_commit_tree = commit::CommitNode::new(into_commit.clone());
                        if commit_tree.clone().is_parent(&into_commit) {
                            println!("Fastforward");
                            let res = branch::set_last_commit(into_branch, commit);
                            if res.is_err() {
                                return Err(RepoError::MergeError);
                            }
                            return Ok(());
                        } else if into_commit_tree.clone().is_parent(&commit) {
                            //nothing
                            return Ok(());
                        } else {
                            println!("Non-fastforward (3way merge)");
                            match commit_tree
                                .clone()
                                .get_most_recent_parent_commit(into_commit_tree.clone())
                            {
                                None => Err(RepoError::CommitError),
                                Some(parent) => {
                                    let parent_index_map =
                                        tree::Tree::tree_to_index_map(parent.tree_hash);
                                    let branch_index_map =
                                        tree::Tree::tree_to_index_map(commit_tree.tree_hash);
                                    let into_branch_index_map =
                                        tree::Tree::tree_to_index_map(into_commit_tree.tree_hash);
                                    match merge::three_fold(
                                        parent_index_map,
                                        branch_index_map,
                                        into_branch_index_map,
                                    ) {
                                        Err(_) => Err(RepoError::MergeError),
                                        Ok(new_tree) => {
                                            match commit::write_commit(
                                                String::from("Merge ")
                                                    + &branch
                                                    + " into "
                                                    + &into_branch,
                                                vec![branch, into_branch.clone()],
                                                new_tree,
                                            ) {
                                                Err(_) => Err(RepoError::CommitError),
                                                Ok(hash) => {
                                                    let file_res = File::create(
                                                        String::from(".yit/refs/heads/")
                                                            + &into_branch,
                                                    );
                                                    match file_res {
                                                        Err(_) => Err(RepoError::IOError),
                                                        Ok(mut file) => {
                                                            let content =
                                                                String::from("ref\n") + &hash;
                                                            let res =
                                                                file.write_all(content.as_bytes());
                                                            if res.is_err() {
                                                                return Err(RepoError::IOError);
                                                            }
                                                            Ok(())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn diff(self, branch1: String, branch2: String) -> Result<(), RepoError> {
        match branch::get_commit(branch1.clone()) {
            Err(_) => Err(RepoError::MergeError),
            Ok(commit1) => {
                let commit1_tree = commit::CommitNode::new(commit1.clone());
                match branch::get_commit(branch2.clone()) {
                    Err(_) => Err(RepoError::MergeError),
                    Ok(commit2) => {
                        let commit2_tree = commit::CommitNode::new(commit2.clone());
                        let commit1_index_map =
                            tree::Tree::tree_to_index_map(commit1_tree.tree_hash);
                        let commit2_index_map =
                            tree::Tree::tree_to_index_map(commit2_tree.tree_hash);
                        let mut result = String::from("");
                        for (key, _) in commit1_index_map.clone() {
                            if commit2_index_map.contains_key(&key) {
                                let diff = diff::get_diff_files(
                                    &commit1_index_map[&key],
                                    &commit2_index_map[&key],
                                );
                                result.push_str(&key);
                                result.push_str(": \n");
                                result.push_str(&diff);
                            }
                        }
                        println!("{}", result);
                        Ok(())
                    }
                }
            }
        }
    }
}
