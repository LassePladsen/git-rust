#[allow(unused_imports)]
use std::env;

mod blob;
mod command;
mod compression;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing command");
        return;
    }

    match args[1].as_str() {
        "init" => command::init(),
        "cat-file" => command::cat_file(&args),
        "hash-object" => command::hash_object(&args),
        cmd => println!("Unknown command: {}", cmd),
    }
}
