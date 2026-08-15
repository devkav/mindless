use std::{collections::HashSet, env, fs::{self}, path::PathBuf, process};
use glob::Pattern;

use crate::util::{constants::{MINDLESS_DIR_NAME, NEVERMIND_FILE_NAME}, output::print_project_not_found};


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


pub fn get_root_or_exit() -> PathBuf {
    let Some(mindless_root) = get_root() else {
        print_project_not_found();
        process::exit(1);
    };

    return mindless_root;
}


pub fn get_nevermind_patterns(mindless_root: &PathBuf) -> Vec<Pattern> {
    // Always ignore .mindless directory
    let sanitized_default_pattern = format!("*/{}", MINDLESS_DIR_NAME);
    let default_pattern = Pattern::new(&sanitized_default_pattern).expect("Something went wrong while creating default pattern");
    let mut patterns: Vec<Pattern> = vec![default_pattern];
    let nevermind_file_path: PathBuf = mindless_root.join(NEVERMIND_FILE_NAME);

    if !nevermind_file_path.exists() || !nevermind_file_path.is_file() {
        return patterns;
    }

    let error_msg = format!("nevermind file at {} could not be read", nevermind_file_path.display());
    let content = fs::read_to_string(nevermind_file_path).expect(&error_msg);

    for line in content.lines() {
        let sanitized_pattern = line.trim()
            .trim_start_matches("/")
            .trim_end_matches("/");
        let sanitized_pattern = format!("*/{}", sanitized_pattern);
        let pattern = Pattern::new(&sanitized_pattern);

        if let Ok(valid_pattern) = pattern {
            patterns.push(valid_pattern);
        }
    }

    return patterns;
}


pub fn get_tracked_files(
    directory: Option<&PathBuf>,
    nevermind_patterns: &Vec<Pattern>,
    mindless_root: &PathBuf
) -> Vec<PathBuf> {
    let directory: &PathBuf = match directory {
        Some(valid_directory) => valid_directory,
        None => &mindless_root
    };

    let mut files: Vec<PathBuf> = Vec::new();
    let children = directory.read_dir().expect("Error searching directory");
    let mut visited: HashSet<PathBuf> = HashSet::new();

    for child in children {
        let Ok(child_path) = child else { continue };
        let child_path = child_path.path();

        if visited.contains(&child_path) {
            continue;
        }

        visited.insert(child_path.clone());

        let mut ignore = false;
        let path_str = child_path.to_str().expect("Something went wrong while reading file path");

        for pattern in nevermind_patterns {
            if pattern.matches(path_str) {
                ignore = true;
                break;
            }
        }

        if ignore {
            continue;
        }

        if child_path.is_dir() {
            let descendants = get_tracked_files(Some(&child_path), nevermind_patterns, &mindless_root);
            files.extend(descendants);
        } else if child_path.is_file() {
            files.push(child_path);
        }
    }

    return files;
}
