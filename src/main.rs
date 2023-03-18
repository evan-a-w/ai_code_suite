use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        process::exit(1);
    }

    let path = &args[1];
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    println!("File: {}", entry.path().display());
                    match fs::read_to_string(entry.path()) {
                        Ok(content) => {
                            println!("Contents:\n```\n{}\n```", content);
                        }
                        Err(err) => {
                            eprintln!("Error reading file {}: {}", entry.path().display(), err);
                        }
                    }
                } else if entry.file_type().is_dir() {
                    println!("Directory: {}", entry.path().display());
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
