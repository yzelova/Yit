use crate::repo;
use std::io;

pub fn read_command() {
    let repo = repo::Repository::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(e) => println!("{}", e),
            Ok(_) => {
                let words: Vec<&str> = input.split_whitespace().collect();
                let command = words[0];
                if command == "init" {
                    match repo.clone().init() {
                        Err(_) => println!("Error initializing."),
                        Ok(_) => println!("Successfully initialized repo."),
                    }
                } else if command == "add" {
                    if words.len() < 2 {
                        println!("Too few arguments!");
                    } else {
                        match repo.clone().add(String::from(words[1])) {
                            Err(_) => println!("Error adding file."),
                            Ok(_) => println!("Successfully added file."),
                        }
                    }
                } else if command == "commit" {
                    if words.len() < 2 {
                        println!("Too few arguments!");
                    } else {
                        match repo.clone().commit(String::from(words[1])) {
                            Err(_) => println!("Error commiting."),
                            Ok(_) => println!("Successful commit."),
                        }
                    }
                } else if command == "checkout" {
                    if words.len() < 2 {
                        println!("Too few arguments!");
                    } else {
                        match repo.clone().checkout(String::from(words[1])) {
                            Err(_) => println!("Error in checkout."),
                            Ok(_) => println!("Successful checkout to {}", words[1]),
                        }
                    }
                } else if command == "merge" {
                    if words.len() < 3 {
                        println!("Too few arguments!");
                    } else {
                        match repo
                            .clone()
                            .merge(String::from(words[1]), String::from(words[2]))
                        {
                            Err(_) => println!("Error merging"),
                            Ok(_) => println!("Successfully merged {} into {}", words[1], words[2]),
                        }
                    }
                } else if command == "diff" {
                    if words.len() < 3 {
                        println!("Too few arguments!");
                    } else {
                        match repo
                            .clone()
                            .diff(String::from(words[1]), String::from(words[2]))
                        {
                            Err(_) => println!("Error merging"),
                            Ok(_) => println!("End diff between {} and {}", words[1], words[2]),
                        }
                    }
                } else if command == "quit" {
                    break;
                }
            }
        }
    }
}
