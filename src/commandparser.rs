use crate::repo;
use std::io::{self, Write};

pub fn read_command() {
    let repo = repo::Repository::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

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
                        println!("Too few arguments! Try: add <file>");
                    } else {
                        match repo.clone().add(String::from(words[1])) {
                            Err(_) => println!("Error adding file."),
                            Ok(_) => println!("Successfully added file."),
                        }
                    }
                } else if command == "commit" {
                    if words.len() < 2 {
                        println!("Too few arguments! Try: commit <message>");
                    } else {
                        match repo.clone().commit(String::from(words[1])) {
                            Err(_) => println!("Error commiting."),
                            Ok(_) => println!("Successful commit."),
                        }
                    }
                } else if command == "checkout" {
                    if words.len() < 2 {
                        println!("Too few arguments! Try: checkout <branch-name>");
                    } else {
                        match repo.clone().checkout(String::from(words[1])) {
                            Err(_) => println!("Error in checkout."),
                            Ok(_) => println!("Successful checkout to {}", words[1]),
                        }
                    }
                } else if command == "merge" {
                    if words.len() < 3 {
                        println!("Too few arguments! Try: merge <branch> <into-branch>");
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
                        println!("Too few arguments! Try: diff <branch1> <branch2>");
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
                } else if command == "help" {
                    println!("Available commands:");
                    println!("  init                            Initialize a new repo");
                    println!("  add      <file>                 Add a new file to be committed");
                    println!("  commit   <message>              Commit the added files");
                    println!("  checkout <branch-name>          Check out the given branch");
                    println!("  merge    <branch> <into-branch> Merge the first branch into the second one");
                    println!("  diff     <branch1> <branch2>    Diff the two branches");
                } else {
                    println!("Unknown command. Try `help` to get a list of valid commands");
                }
            }
        }
    }
}
