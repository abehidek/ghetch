use super::features;

use serde::Deserialize;
use serde_json::{Number, Value};

#[derive(Debug, Deserialize)]
struct RepositoryResponse {
    id: Number,
    name: String,
    created_at: String,
    stargazers_count: Number,
    watchers_count: Number,
    language: String,
    default_branch: String,
}

#[derive(Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
    date: String,
}

#[derive(Deserialize, Debug)]
struct Commit {
    message: String,
    author: Author,
}

#[derive(Deserialize, Debug)]
struct RepositoryLastCommitResponse {
    sha: String,
    commit: Commit,
}

#[derive(Deserialize, Debug)]
struct Languages {}

pub async fn fetch(user_arg: &str) {
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
    let url_repo: String = format!("https://api.github.com/repos/{}/{}", vec[0], vec[1]);
    let url_repo_last_commit: String = format!("{}/commits?per_page=1", url_repo);
    let url_repo_langs: String = format!("{}/languages", url_repo);

    let repo_str = features::fetch_api(url_repo).await;
    let repo: RepositoryResponse = serde_json::from_str(&repo_str).expect("Bad json");

    let last_commit_str = features::fetch_api(url_repo_last_commit).await;
    let last_commit: Vec<RepositoryLastCommitResponse> =
        serde_json::from_str(&last_commit_str).expect("Bad json");

    let repo_langs_str = features::fetch_api(url_repo_langs).await;
    let repo_langs_val: serde_json::Value =
        serde_json::from_str(&repo_langs_str).expect("Bad json");
    let mut default_repo_langs = serde_json::Map::new();
    default_repo_langs.insert(
        "no lang".to_string(),
        serde_json::Value::Number(serde_json::Number::from(0)),
    );
    let repo_langs = repo_langs_val.as_object().unwrap_or(&default_repo_langs);

    // match repo_langs {
    //     Some(value) => {
    //         // println!("{:?}", value);
    //         for (key, value) in value.into_iter() {
    //             println!("{} / {}", key, value)
    //         }
    //     }
    //     None => {
    //         println!("Nao existe");
    //     }
    // }

    print_general(repo, last_commit);
    print_langs(repo_langs);
}

fn print_general(repo: RepositoryResponse, last_commit: Vec<RepositoryLastCommitResponse>) {
    println!(
        "
General:

    id: {}
    name: {}
    stars: {}
    watchers: {}
    created at: {}
    last commit: {:?}
    ",
        repo.id,
        repo.name,
        repo.stargazers_count,
        repo.watchers_count,
        repo.created_at,
        last_commit
    );
}

fn print_langs(repo_langs: &serde_json::Map<String, Value>) -> () {
    println!(
        "
Repository Langs"
    );
    for (key, value) in repo_langs.into_iter() {
        print!(
            "
    {} / {}",
            key, value
        )
    }
}
