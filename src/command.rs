use std::fs::{self, File};
use std::io::{Write, stdout};

use anyhow::{Error, Result, bail, ensure};

use crate::object::{self, Object, ObjectKind, blob};

pub type Output = Vec<u8>;

/// Init git dir
pub fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

/// Read blob
pub fn cat_file(args: &[String]) -> Output {
    // Get blob hash from positional arg
    let mut hash: Option<&str> = None;
    for arg in &args[2..] {
        // Flag argument, skip for now. TODO: support flags?
        if arg.starts_with('-') {
            continue;
        }
        hash = Some(arg);
    }
    let Some(hash) = hash else {
        return "Missing hash\n".into();
    };
    if hash.len() < 3 {
        return "Hash name too short\n".into();
    }
    let object = match Object::read(hash) {
        Ok(object) => object,
        Err(e) => return format!("{e}").into(),
    };
    object.contents
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

/// Inspect tree object
pub fn ls_tree(args: &[String]) {
    // Get tree hash input from positional arg
    let mut hash: Option<&str> = None;
    let mut print_name_only = false;
    for arg in &args[2..] {
        // Supports flag: --name-only
        if "--name-only" == arg {
            print_name_only = true;
        }
        if arg.starts_with('-') {
            continue;
        }
        hash = Some(arg);
    }
    let Some(hash) = hash else {
        println!("Missing hash");
        return;
    };

    // TODO: also support full print (where print_name_only=false)

    // Read file
    let path = object::get_path(hash);
    // let file = File
}
