use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

fn main() {
    // Check if an argument was provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an argument.");
        exit(1);
    }

    // Get the directory of the current executable
    let script_dir = env::current_exe()
        .expect("Failed to get current executable path")
        .parent() // Get the parent directory
        .expect("Failed to get parent directory")
        .to_path_buf(); // Convert to PathBuf

    // Search for the plantuml*.jar file in the current directory
    let jar_file = find_jar_file(&script_dir).expect("Failed to find JAR file");

    // Run the Java program with the found JAR file and the provided argument
    let status = Command::new("java")
        .arg("-Dfile.encoding=UTF-8")
        .arg("-jar")
        .arg(&jar_file)
        .arg("-tsvg")
        .arg("-charset")
        .arg("UTF-8")
        .arg(&args[1])
        .status()
        .expect("Failed to execute Java command");

    // Exit with the status of the Java command
    if status.success() {
        exit(0);
    } else {
        eprintln!("Java command failed with status: {:?}", status);
        exit(1);
    }
}

// Function to find the JAR file in the specified directory
fn find_jar_file(dir: &Path) -> Option<PathBuf> {
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        // Check if the file name matches the pattern "plantuml*.jar"
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                if name_str.starts_with("plantuml") && name_str.ends_with(".jar") {
                    return Some(path);
                }
            }
        }
    }

    None
}
