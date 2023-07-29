use std::{fs, path::Path};

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
                if f.path().is_dir() || f.path().is_symlink() {
                    parse_dir(files, &f.path().display().to_string().to_owned(), detph - 1);
                }else{
                    match get_file_type(f.path().display().to_string().to_owned()) {
                        Some(ext) => {
                            if ext == "c" || ext == "h" || ext == "cpp" || ext == "hpp" {
                                 files.push(f.path().display().to_string()[2..].to_owned());
                            }
                        },
                        None => { }
                    } 
                }
            },
            Err(_) => { }
        }
    }
}

fn get_file_type(file_name: String) -> Option<String> {
    let path = Path::new(&file_name);
    if path.exists() {
        if let Some(ext) = path.extension() {
            return Some(ext.to_string_lossy().into_owned());
        }
    }
    None
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
                if f.path().is_dir() || f.path().is_symlink() {
                    parse_dir(&mut src_files, &f.path().display().to_string().to_owned(), depth.clone());
                }else{
                    match get_file_type(f.path().display().to_string().to_owned()) {
                        Some(ext) => {
                            if ext == "c" || ext == "h" || ext == "cpp" || ext == "hpp" {
                                 src_files.push(f.path().display().to_string()[2..].to_owned());
                            }
                        },
                        None => { }
                    } 
                }
            },
            Err(_) => { }
        }
    }

    print!("CC = gcc\n\nCFLAGS := -Wall -Wextra -std=c17\n\n SRC := ");
    for src in src_files {
        print!("{} \\\n\t", src);
    }
    print!("\n\n TARGET := a\n\n");
    print!("all: $(TARGET)\n\n$(TARGET): $(SRC)\n\t$(CC) $(CFLAGS) $(SRC) -o $(TARGET)\n\n");
    print!("clean:\n\trm -f $(TARGET)\n");
}
