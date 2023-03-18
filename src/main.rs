use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use walkdir::WalkDir;

fn is_source_code_file(path: &std::path::Path) -> bool {
    // List of source code file extensions
    let source_code_extensions = [
        "c", "cpp", "h", "hpp", "rs", "java", "py", "js", "ts", "go", "cs", "rb", "php", "swift",
    ];

    match path.extension() {
        Some(extension) => source_code_extensions.contains(&extension.to_str().unwrap_or("")),
        None => false,
    }
}

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
                    let file_path = entry.path();
                    if is_source_code_file(file_path) {
                        println!("File: {}", file_path.display());
                        match fs::read_to_string(file_path) {
                            Ok(content) => {
                                println!("Contents:\n{}", content);
                            }
                            Err(err) => {
                                eprintln!(
                                    "Error reading file {}: {}",
                                    file_path.display(),
                                    err
                                );
                            }
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
