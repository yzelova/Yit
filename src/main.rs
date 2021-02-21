mod blob;
mod branch;
mod commit;
mod file;
mod index;
mod repo;
mod tree;
mod merge;
mod commandparser;
mod diff;

fn main() {
    commandparser::read_command();
    // let repo = repo::Repository::new(String::from(
    //     "C:\\Users\\Yoana\\Desktop\\FMI\\3rd Semester\\Rust\\yit",
    // ));
    // let _ = repo.clone().init();
    // let res = repo.clone().add(String::from("foobar"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok"),
    // }
    // let res = repo.clone().commit(String::from("saobshtenie master 1"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok"),
    // }
    // let res = repo.clone().checkout(String::from("master"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok"),
    // }
    // let res = repo.clone().add(String::from("ehoo\\daaa"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok"),
    // }
    // let res = repo.clone().commit(String::from("saobshtenie branch1"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok"),
    // }
    // let res = repo.clone().checkout(String::from("master"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok")
    // }
    // let res = repo.clone().add(String::from("src\\tree.rs"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok")
    // }
    // let res = repo.clone().commit(String::from("saobshtenie master 1"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok")
    // }
    // let res = repo::Repository::merge(String::from("master"), String::from("branch1"));
    // match res {
    //     Err(_) => println!("err"),
    //     Ok(_) => println!("ok")
    // }

    //let res = tree::Tree::tree_to_index_map(String::from("48fb0738f7f08b5a2e5f057ffeb6b2ab5e7c68d1"));

    //commit
    // let commit1 = String::from("1e52111474cc8950283fabbffc08312fb6b249bb");
    // //commit
    //let commit2 = String::from("b697dae82ae6c78cbd4f0b159a195da68398f125");

    // let commitTree1 = commit::CommitNode::new(commit2);
    // commitTree1.print();
}

