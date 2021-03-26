use yit::repo;
use std::{fs, env};

#[test]
fn test_flow() {
    // Reset "tmp" directory:
    let _ = fs::remove_dir_all("tmp");
    fs::create_dir("tmp").unwrap();
    fs::write("tmp/.gitkeep", b"").unwrap();

    // Create initial state:
    env::set_current_dir("tmp").unwrap();
    fs::write("foobar", b"test content").unwrap();
    fs::create_dir("ehoo").unwrap();
    fs::write("ehoo/daaa", b"another test").unwrap();
    fs::create_dir("src").unwrap();
    fs::write("src/tree.rs", b"fn rust_code() {}").unwrap();

    let repo = repo::Repository::new();
    let _ = repo.clone().init();
    let res = repo.clone().add(String::from("foobar"));
    assert_eq!(true, res.is_ok());
    let res = repo.clone().commit(String::from("message master 1"));
    assert_eq!(true, res.is_ok());
    let res = repo.clone().checkout(String::from("branch1"));
    assert_eq!(true, res.is_ok());
    let res = repo.clone().add(String::from("ehoo/daaa"));
    assert_eq!(true, res.is_ok());
    let res = repo.clone().commit(String::from("message branch1"));
    assert_eq!(true, res.is_ok());
    let res = repo.clone().checkout(String::from("master"));
    assert_eq!(true, res.is_ok());
    let res = repo.clone().add(String::from("src/tree.rs"));
    assert_eq!(true, res.is_ok());
    let res = repo.clone().commit(String::from("message master 2"));
    assert_eq!(true, res.is_ok());
    let res = repo
        .clone()
        .diff(String::from("master"), String::from("branch1"));
    assert_eq!(true, res.is_ok());
    let res = repo.merge(String::from("master"), String::from("branch1"));
    assert_eq!(true, res.is_ok());
}
