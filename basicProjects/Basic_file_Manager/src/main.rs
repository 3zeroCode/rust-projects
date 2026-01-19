use std::fs;
use std::io::{self, Write};

fn main() {
    loop {
        println!("Welcome to the file system program");
        println!("Press 1 to list all files");
        println!("Press 2 to copy files");
        println!("Press 3 to exit program");
        print!("Enter your Selection: ");
        io::stdout().flush().ok();

        let mut user_choice = String::new();
        if io::stdin().read_line(&mut user_choice).is_err() {
            println!("Error reading input, please try again");
            continue;
        }

        match user_choice.trim() {
            "1" => list_files(),
            "2" => copy_files(),
            "3" => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid selection, please enter 1, 2, or 3."),
        }
    }
}

fn list_files() {
    println!("Enter directory (or . for current):");
    let mut user_path = String::new();
    if io::stdin().read_line(&mut user_path).is_err() {
        println!("Failed to read input");
        return;
    }
    let path = user_path.trim();

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => match entry.metadata() {
                        Ok(metadata) => {
                            let file_type = if metadata.is_dir() { "[DIR]" } else { "[FILE]" };
                            println!("{} {}", file_type, entry.path().display());
                        }
                        Err(e) => println!("Could not read metadata: {}", e),
                    },
                    Err(e) => println!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory '{}': {}", path, e),
    }
}

fn copy_files() {
    println!("Enter source file path:");
    let mut src = String::new();
    if io::stdin().read_line(&mut src).is_err() {
        println!("Failed to read input");
        return;
    }
    let src = src.trim();

    println!("Enter destination path:");
    let mut dest = String::new();
    if io::stdin().read_line(&mut dest).is_err() {
        println!("Failed to read input");
        return;
    }
    let dest = dest.trim();

    match fs::copy(src, dest) {
        Ok(bytes) => println!("Copied {} bytes from '{}' to '{}'.", bytes, src, dest),
        Err(e) => println!("Failed to copy file: {}", e),
    }
}

