mod features;
use std::env;
use tokio;

async fn match_args(args: Vec<String>) {
    /*
        need simpliciation and support for easy to add feature
        minimalist version of onefetch
    */

    for (arg_index, arg) in args.iter().enumerate() {
        match &arg[..] {
            "-r" | "--repo" | "--repository" => {
                if !args.len() >= arg_index + 1 {
                    features::fetch_repo(&args[arg_index + 1][..]).await;
                    return;
                }
            }
            "-u" | "--user" => {
                if args.len() >= arg_index + 1 {
                    features::fetch_user(&args[arg_index + 1][..]).await;
                    return;
                }
            }
            _ => {}
        }
    }
    println!("Unknow command");
    return;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match_args(args).await;
    } else {
        features::help("");
    }
    Ok(())
}
