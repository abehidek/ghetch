use std::env::{self, Args};

fn fetch_user(username: &str) {
    println!("[log] fetching user {}", username);
    /*
        needs to accept both url and "username" formats
        also, needs to count the lines and create a language bar for the user (most used langs)
    */
}

fn fetch_repo(reponame: &str) {
    println!("[log] fetching repository {}", reponame);

    /*
        needs to accept both url and "username/reponame" formats
        also, needs to count the lines and create a language bar for the repo
    */
}

fn match_args(args: Vec<String>, features: &Vec<Feature>) {
    /*
        need simpliciation and support for easy to add feature
        minimalist version of onefetch
    */

    for (index, arg) in args.iter().enumerate() {
        // match &&arg[..] {
        //     &"-r" | &"--repository" => {
        //         if args.len() <= index + 1 {
        //             println!("Which repo?");
        //             return;
        //         }
        //         let reponame = &args[index + 1][..];
        //         fetch_repo(reponame);
        //         return;
        //     }
        //     &"-u" | &"--user" => {
        //         if args.len() <= index + 1 {
        //             println!("Which user?");
        //             return;
        //         }
        //         let username = &args[index + 1][..];
        //         fetch_user(username);
        //         return;
        //     }
        //     &&_ => {
        //         println!("Unknow command");
        //         return;
        //     }
        // }
    }
}

struct Feature {
    name: String,
    description: String,
    flags: Vec<String>,
    function: fn(&str),
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let features: Vec<Feature> = vec![
        Feature {
            name: "fetch-repo".to_string(),
            description: "Fetch a github repo".to_string(),
            flags: vec!["-r".to_string(), "--repository".to_string()],
            function: fetch_repo,
        },
        Feature {
            name: "fetch-user".to_string(),
            description: "Fetch a github user".to_string(),
            flags: vec!["-u".to_string(), "--user".to_string()],
            function: fetch_user,
        },
    ];

    if args.len() > 1 {
        match_args(args, &features);
    } else {
        println!("commands: ")
    }
}
