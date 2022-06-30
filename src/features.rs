use std::process::Output;

use futures::Future;

pub struct Feature {
    pub name: String,
    pub description: String,
    pub necessary_args: String,
    pub flags: Vec<String>,
    pub function: fn(&str),
}

pub fn get() -> Vec<Feature> {
    vec![
        Feature {
            name: "fetch-repo".to_string(),
            description: "Fetch a github repo".to_string(),
            necessary_args: "username-reponame or repo url".to_string(),
            flags: vec![
                "-r".to_string(),
                "--repository".to_string(),
                "--repo".to_string(),
            ],
            function: fetch_repo,
        },
        Feature {
            name: "fetch-user".to_string(),
            description: "Fetch a github user".to_string(),
            necessary_args: "username or user url".to_string(),
            flags: vec!["-u".to_string(), "--user".to_string()],
            function: fetch_user,
        },
        Feature {
            name: "help".to_string(),
            description: "All commands to help user".to_string(),
            necessary_args: "".to_string(),
            flags: vec!["-h".to_string(), "--help".to_string()],
            function: help,
        },
    ]
}

enum FetchErrors {
    Failed,
}

fn fetch_user(username: &str) {
    println!("[log] fetching user {}", username);
    /*
        needs to accept both url and "username" formats
        also, needs to count the lines and create a language bar for the user (most used langs)
    */
}

// async fn fetch_api(url: String) {
//     let reqwest_client = reqwest::Client::new();
//     let response = reqwest_client
//         .get(url)
//         .header(USER_AGENT, "rust")
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .expect("Fetch failed");
//     print!("Response: {}", response.to_string());
// }

fn fetch_repo(user_arg: &str) {
    println!("[log] fetching repository {}", user_arg);

    if !user_arg.contains("-") {
        println!("Invalid reponame");
        return;
    };

    let vec: Vec<&str> = user_arg.split("-").collect();
    let url: String = format!("https://api.github.com/users/{}/{}", vec[0], vec[1]);
    /*
        needs to accept both url and "username-reponame" formats
        also, needs to count the lines and create a language bar for the repo
    */
}

pub fn help(_str: &str) {
    println!(
        "USAGE: ghetch [options]
    \nOPTIONS:"
    );
    for feature in get() {
        print!("    {:?}", feature.flags);
        if !feature.necessary_args.is_empty() {
            println!(" necessary args: {}", feature.necessary_args)
        }
    }
}
