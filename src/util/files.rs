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


pub fn get_tracked_files() -> Vec<PathBuf> {
    let files: Vec<PathBuf> = Vec::new();

    // TODO
    let root_result = get_root();

    if root_result.is_none() {
        // TODO: Fix err
        //return Err("mindless project root not found.");
    }

    let root = root_result.unwrap();

    return files;
}
