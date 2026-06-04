use std::env;

mod command;
mod compression;
mod object;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing command");
        return;
    }

    object::Object::new(&args[2]);

    match args[1].as_str() {
        "init" => command::init(),
        "cat-file" => command::cat_file(&args),
        "hash-object" => command::hash_object(&args),
        "ls-tree" => command::ls_tree(&args),
        cmd => println!("Unknown command: {}", cmd),
    }
}
