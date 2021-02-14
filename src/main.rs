mod repo;

fn main() {
    let repo = repo::Repository::new(String::from("C:\\Users\\Yoana\\Desktop\\FMI\\3rd Semester\\Rust\\yit"));
    let _ = repo.clone().init();
    let res = repo.add(String::from("foobar"));
    match res {
        Err(_) => println!("err"),
        Ok(_) => println!("ok")
    }
}
