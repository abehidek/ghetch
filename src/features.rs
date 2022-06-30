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
            necessary_args: "repository name".to_string(),
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
            necessary_args: "user name".to_string(),
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

pub fn help(_str: &str) {
    println!(
        "USAGE: ghetch [options]
    \nOPTIONS:"
    );
    for feature in get() {
        print!("    {:?}", feature.flags);
        if !feature.necessary_args.is_empty() {
            println!("[args: {}]", feature.necessary_args)
        }
    }
}
