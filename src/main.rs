use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let target_path = args.get(1).map_or(".", |s| s.as_str());

    let files = list_files(target_path)?;

    for file_path in &files {
        println!("Processing file: {:?}", file_path);
        if let Err(e) = read_and_display_file(file_path) {
            eprintln!("Error reading file: {:?}: {}", file_path, e);
        }
    }

    Ok(())
}

fn list_files(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();
    visit_dirs(Path::new(dir), &mut file_paths)?;
    file_paths.sort();
    
    Ok(file_paths)
}

fn visit_dirs(dir: &Path, file_paths: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, file_paths)?;
            } else {
                file_paths.push(path);
            }
        }
    }

    Ok(())
}

fn read_and_display_file(file_path: &PathBuf) -> io::Result<()> {
    let mut f = File::open(file_path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    for line in content.lines() {
        println!("{}", line);
    }

    Ok(())
}
