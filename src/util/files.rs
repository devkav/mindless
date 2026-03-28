use std::{env, path::{PathBuf}};

use crate::util::constants::MINDLESS_DIR_NAME;


pub fn get_root() -> Option<PathBuf> {
    let mut current_directory = env::current_dir().expect("Couldn't find working directory");
    let mut system_root_reached = false;

    while !system_root_reached {
        let mindless_dir = current_directory.join(MINDLESS_DIR_NAME);

        if mindless_dir.exists() {
            return Some(current_directory);
        }

        match current_directory.parent() {
            Some(parent_directory) => {
                current_directory = parent_directory.to_path_buf();
            },
            None => {
                system_root_reached = true;
            }
        }
    }

    return None;
}


pub fn get_tracked_files(directory: Option<PathBuf>) -> Result<Vec<PathBuf>, &'static str> {
    let directory: PathBuf = if directory.is_none() {
        let root = get_root();

        if root.is_none() {
            return Err("mindless project root not found.");
        }

        root.unwrap()
    } else {
        directory.unwrap()
    };

    let mut files: Vec<PathBuf> = Vec::new();
    let children = directory.read_dir().expect("Error searching directory");

    for child in children {
        let Ok(valid_path) = child else { continue };
        let current_path = valid_path.path();

        if current_path.is_dir() {
            let descendants_result = get_tracked_files(Some(current_path));

            if let Ok(descendants) = descendants_result {
                files.extend(descendants);
            }
        } else if current_path.is_file() {
            // TODO: Check if file is in .nevermind
            files.push(current_path);
        }
    }


    return Ok(files);
}
