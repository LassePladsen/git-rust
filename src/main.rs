#[allow(unused_imports)]
use std::env;

mod blob;
mod command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing command");
        return;
    }

    match args[1].as_str() {
        "init" => command::init(),
        "cat-file" => command::cat_file(&args),
        cmd => println!("Unknown command: {}", cmd),
    }
}
