pub async fn fetch(username: &str) {
    /*
        needs to accept both url and "username" formats
        also, needs to count the lines and create a language bar for the user (most used langs)
    */
    println!("[log] fetching user {}", username);

    let url: String = format!("https://api.github.com/users/{}", username);
}
