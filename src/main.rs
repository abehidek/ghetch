mod features;
use std::env;

fn match_args(args: Vec<String>, features: &Vec<features::Feature>) {
    /*
        need simpliciation and support for easy to add feature
        minimalist version of onefetch
    */

    for (arg_index, arg) in args.iter().enumerate() {
        for feature in features.iter() {
            // println!("[debug] arg {}, feature {}", arg, feature.name);
            if feature.flags.contains(arg) {
                println!("running feature: {}", feature.name);
                let feature_function: fn(&str) = feature.function;

                if !feature.necessary_args.is_empty() {
                    if args.len() <= arg_index + 1 {
                        println!("You need to pass {}", feature.necessary_args);
                        return;
                    }
                    feature_function(&args[arg_index + 1][..]);
                } else {
                    feature_function("");
                }
                return;
            }
        }
    }
    println!("Unknow command");
    return;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let features: Vec<features::Feature> = features::get();

    if args.len() > 1 {
        match_args(args, &features);
    } else {
        features::help("");
    }
}
