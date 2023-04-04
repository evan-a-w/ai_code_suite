use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use walkdir::WalkDir;
use regex::Regex;

fn is_source_code_file(path: &std::path::Path) -> bool {
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
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <path> [ignore_pattern]", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    let ignore_regex = args.get(2).and_then(|s| Regex::new(s).ok());

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                let file_path = entry.path();
                let file_path_str = file_path.to_str().unwrap_or("");
                if ignore_regex.as_ref().map(|r| r.is_match(file_path_str)).unwrap_or(false) {
                    continue;
                }

                if entry.file_type().is_file() {
                    if is_source_code_file(file_path) {
                        println!("{}", file_path.display());
                        match fs::read_to_string(file_path) {
                            Ok(content) => {
                                println!("```\n{}\n```", content);
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
