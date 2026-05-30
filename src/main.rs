#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("LP args: {args:?}");
    
    if args.len() < 2 {
        println!("missing command");
        return;
    }

    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(&args),
        cmd @_ => println!("unknown command: {}", cmd),
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

fn cat_file(args: &[String]) {
    let mut path: Option<&str> = None;
    for arg in args {
        // Flag, skip for now.
        if '-' == arg.chars().nth(0).unwrap() {
            continue;
        }
        path = Some(arg);
    }
    println!("LP path: {path:?}");
    if path.is_none() {
        println!("missing path");
    }
    let path = path.unwrap();


    
    
}
