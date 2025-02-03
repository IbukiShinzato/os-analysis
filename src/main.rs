use std::fs::{self, File};
use std::io::Read;
use std::{env::args, io, path::PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    let mut path = ".";

    if args.len() > 1 {
        path = args[1].as_str();
    }

    let dirs = visit_dirs(path)?;

    for file_path in &dirs {
        println!("{:?}", file_path);
        read_file(file_path)?;
    }
    Ok(())
}

fn visit_dirs(path: &str) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    Ok(entries)
}

fn read_file(file_path: &PathBuf) -> io::Result<()> {
    let mut f = File::open(file_path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    let data: Vec<&str> = buffer.split('\n').collect();
    for d in data {
        println!("{}", d);
    }

    Ok(())
}
