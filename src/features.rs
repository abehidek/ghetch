use reqwest::header::USER_AGENT;

pub async fn fetch_api(url: String) -> String {
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
    response.to_string()
}

pub fn help(_str: &str) {
    println!(
        "USAGE: ghetch [options]
    \nOPTIONS:"
    );
}
