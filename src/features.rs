use reqwest::header::{AUTHORIZATION, USER_AGENT};

async fn fetch_api(url: String) {
    println!("[log][fetch_api] fetching api: {}", url);
    let reqwest_client = reqwest::Client::new();
    let response = reqwest_client
        .get(url)
        .header(USER_AGENT, "rust")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .expect("Fetch failed");
    print!("Response: {}", response.to_string());
}

pub async fn fetch_user(username: &str) {
    /*
        needs to accept both url and "username" formats
        also, needs to count the lines and create a language bar for the user (most used langs)
    */
    println!("[log] fetching user {}", username);

    let url: String = format!("https://api.github.com/users/{}", username);
    fetch_api(url).await;
}

pub async fn fetch_repo(user_arg: &str) {
    /*
        needs to accept both url and "username-reponame" formats
        also, needs to count the lines and create a language bar for the repo
    */
    println!("[log] fetching repository {}", user_arg);

    if !user_arg.contains("-") {
        println!("Invalid reponame");
        return;
    };

    let vec: Vec<&str> = user_arg.split("-").collect();
    let url: String = format!("https://api.github.com/repos/{}/{}", vec[0], vec[1]);
    fetch_api(url).await;
}

pub fn help(_str: &str) {
    println!(
        "USAGE: ghetch [options]
    \nOPTIONS:"
    );
}
