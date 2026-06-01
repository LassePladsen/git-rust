use crate::blob;
use std::fs;
use std::io::{Write, stdout};

/// Init git dir
pub fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

/// Read blob
pub fn cat_file(args: &[String]) {
    // Get path from positional arg
    let mut path: Option<&str> = None;
    for arg in &args[2..] {
        // Flag argument, skip for now. TODO: support flags?
        if arg.starts_with('-') {
            continue;
        }
        path = Some(arg);
    }
    let Some(path) = path else {
        println!("Missing path");
        return;
    };

    // Find path to objects dir. path e3123456 is .git/objects/e3/123456
    if path.len() < 3 {
        println!("Path name too short");
        return;
    }
    let dir = &path[0..2];
    let filename = &path[2..];
    let path = format!(".git/objects/{dir}/{filename}");

    // Write blob contents to stdout
    let contents = blob::read_blob(&path).expect("Could not read blob");
    let _ = stdout().write_all(&contents);
}

/// Hash object to blob
pub fn hash_object(args: &[String]) {
    // Get path from positional arg
    let mut path: Option<&str> = None;
    for arg in &args[2..] {
        // Flag argument, skip for now. TODO: support flags?
        if arg.starts_with('-') {
            continue;
        }
        path = Some(arg);
    }
    let Some(path) = path else {
        println!("Missing path");
        return;
    };

    // Read file
    let bytes = match fs::read(path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    blob::write_blob(&bytes).expect("Could not write blob");
}
