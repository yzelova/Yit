mod repo;

fn main() {
    let repo = repo::Repository::new(String::from("C:\\Users\\Yoana\\Desktop\\FMI\\3rd Semester\\Rust\\yit"));
    //let _ = repo.clone().init();
    let res = repo.clone().add(String::from("src\\repo\\tree.rs"));
    match res {
        Err(_) => println!("err"),
        Ok(_) => println!("ok")
    }

    let res = repo.commit(String::from("First commit"), Vec::new());
    match res {
        Err(_) => println!("err"),
        Ok(_) => println!("ok")
    }
}
