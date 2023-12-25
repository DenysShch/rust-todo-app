use std::env;
use std::fs;
use std::path::Path;

pub trait FileSystem {
    fn home_dir() -> String {
        return env::var("HOME").expect("$HOME dir not exist");
    }

    fn get_path() -> String {
        let path = format!("{}/.todo", Self::home_dir());
        if !Path::new(&path).is_dir() {
            let _ = fs::create_dir(&path);
        }
        return path;
    }

    fn check_if_file_exist(path: String) -> bool {
        match fs::metadata(path) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
