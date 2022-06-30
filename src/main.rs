use std::env::{self};

fn fetch_user(username: &str) {
    println!("[log] fetching user {}", username);
}

fn fetch_repo(reponame: &str) {
    println!("[log] fetching repository {}", reponame);
}

fn match_args(args: Vec<String>) {
    for (index, arg) in args.iter().enumerate() {
        match &&arg[..] {
            &"-r" | &"--repository" => {
                if args.len() <= index + 1 {
                    println!("Which repo?");
                    return;
                }
                let reponame = &args[index + 1][..];
                fetch_repo(reponame);
                return;
            }
            &"-u" | &"--user" => {
                if args.len() <= index + 1 {
                    println!("Which user?");
                    return;
                }
                let username = &args[index + 1][..];
                fetch_user(username);
                return;
            }
            &&_ => {
                println!("Unknow command");
                return;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match_args(args);
    } else {
        println!("commands: ")
    }
}
