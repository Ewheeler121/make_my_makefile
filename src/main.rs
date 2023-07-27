use std::{fs, fmt, fs::File};
use std::io::Write;

#[derive(PartialEq)]
enum Compiler {
    GCC,
    GPP
}

impl fmt::Display for Compiler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &Compiler::GPP {
            return write!(f, "g++")
        }else{
            return write!(f, "gcc")
        }
    }
}

#[derive(Debug, PartialEq)]
enum FileType<String> {
   C(String),
   H(String),
   CPP(String),
   HPP(String),
   O(String),
   RC(String),
   Unknown(())
}

impl FileType<String> {
   pub fn get_file_type(file_name: String) -> FileType<String> {
        if file_name.contains(".c") {
            return FileType::C(file_name);
        }

        if file_name.contains(".h") {
            return FileType::H(file_name);
        }

        if file_name.contains(".cpp") {
            return FileType::CPP(file_name);
        }

        if file_name.contains(".hpp") {
            return FileType::HPP(file_name); 
        }

        if file_name.contains(".o") {
            return FileType::O(file_name);
        }
        
        if file_name.contains(".rc") {
            return FileType::RC(file_name);
        }

        FileType::Unknown(())
   }
}

fn parse_dir(files: &mut Vec<FileType<String>>, dir: &String, detph: i32) {
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
                    files.push(FileType::get_file_type(f.path().display().to_string()[2..].to_owned()));
                }
            },
            Err(err) => {
                eprintln!("ingoring file: {}", err); 
            }
        }
    }
}

fn main() {
    let mut complier: Compiler = Compiler::GCC;
    let mut src_files: Vec<FileType<String>> = vec![]; 
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
                    parse_dir(&mut src_files, &f.path().display().to_string(), depth);
                }else{ 
                    let compiler = match FileType::get_file_type(f.path().display().to_string()[2..].to_owned()) {
                       FileType::C(file_name) => {
                           
                       },
                       FileType::H(file_name) => {
                       
                       },
                       FileType::CPP(file_name) => {
                            if complier != Compiler::GPP {
                                return Compiler::GPP
                            }
                       },
                       FileType::HPP(file_name) => {

                       },
                       FileType::O(file_name) => {

                       },
                       FileType::RC(file_name) => todo!(),
                       FileType::Unknown(_) => {}
                    };
                    //src_files.push(FileType::get_file_type(f.path().display().to_string()[2..].to_owned()));
                }
            },
            Err(err) => {
                eprintln!("ingoring file: {}", err); 
            }
        }
    } 
} 
