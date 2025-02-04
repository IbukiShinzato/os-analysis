use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let target_path = args.get(1).map_or(".", |s| s.as_str());

    let files = get_file_list(target_path)?;

    let mut submission_data: HashMap<String, Vec<usize>> = HashMap::new();

    for file_path in &files {
        println!("Processing file: {:?}", file_path);
        if let Err(e) = read_file(file_path, &mut submission_data) {
            eprintln!("Error reading file: {:?}: {}", file_path.display(), e);
        }
    }

    for (number, submissions) in submission_data
        .iter()
        .collect::<Vec<_>>()
        .into_iter()
        .sorted_by_key(|&(k, _)| k)
    {
        println!("{}: {:?}", number, submissions);
    }

    Ok(())
}

fn get_file_list(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();
    collect_files_recursively(&PathBuf::from(dir), &mut file_paths)?;
    file_paths.sort();

    println!("{:?}", file_paths);
    Ok(file_paths)
}

fn collect_files_recursively(dir: &PathBuf, file_paths: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_files_recursively(&path, file_paths)?;
            } else {
                file_paths.push(path);
            }
        }
    }

    Ok(())
}

// Reads a file and updates the submission data with parsed results
fn read_file(
    file_path: &PathBuf,
    submission_data: &mut HashMap<String, Vec<usize>>,
) -> io::Result<()> {
    let mut f = File::open(file_path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    for line in content.lines() {
        let data: Vec<&str> = line.split(' ').collect();
        let number = data[0];
        let success = data[1].parse::<usize>().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid data: {}", e))
        })?;

        submission_data
            .entry(number.to_string())
            .or_insert_with(Vec::new)
            .push(success)
    }

    Ok(())
}
