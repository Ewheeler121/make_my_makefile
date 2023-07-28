use std::{fs, fmt, fs::File};
use std::io::Write;

fn parse_dir(files: &mut Vec<String>, dir: &String, detph: i32) {
    let current_path: fs::ReadDir = match fs::read_dir(&dir) {
        Ok(path) => path,
        Err(_) => {
            panic!("Could not open {}", dir);
        }
    };

    for file in current_path {
        match file {
            Ok(f) => {
                if f.path().is_dir() && 0 < detph {
                    parse_dir(files, &dir, detph - 1);
                }else{
                    files.push(f.path().display().to_string()[2..].to_owned());
                }
            },
            Err(err) => {
                eprintln!("ingoring file: {}", err); 
            }
        }
    }
}

fn main() {
    let mut src_files: Vec<String> = vec![]; 
    let depth: i32 = 3;

    let current_path: fs::ReadDir = match fs::read_dir("./") {
        Ok(path) => path,
        Err(_) => {
            panic!("Could not open current directory");
        }
    };

    for file in current_path {
        match file {
            Ok(f) => {
                if f.path().is_dir() {
                    parse_dir(&mut src_files, &f.path().display().to_string()[2..].to_owned(), depth.clone());
                }else{
                    src_files.push(f.path().display().to_string()[2..].to_owned());
                }
            },
            Err(_) => { }
        }
    }
} 
