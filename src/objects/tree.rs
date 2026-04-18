use std::{collections::{HashMap, HashSet}, path::PathBuf};

use crate::{objects::{blob::create_blob, object::create_object}, util::constants::{BLOB_OBJECT_TYPE, TREE_OBJECT_TYPE}};


pub fn create_tree(path_prefix: &PathBuf, mindless_root: &PathBuf, tracked_files: &Vec<PathBuf>) -> String {
    let mut visited = HashSet::new();
    let mut tree_hashes: HashMap<String, String> = HashMap::new();
    let mut blob_hashes: HashMap<String, String> = HashMap::new();
    
    for file in tracked_files {
        if !file.starts_with(path_prefix) {
            continue;
        }

        let relative_path = file.strip_prefix(path_prefix).expect("Something went wrong while stripping root prefix");
        let current_object_option = relative_path.components().next();

        if let Some(current_object) = current_object_option {
            let current_object_path = path_prefix.join(current_object);
            let current_object_str = current_object
                .as_os_str()
                .to_str()
                .expect("Something went wrong while converting to a string")
                .to_owned();

            if current_object_path.is_dir() {
                if !visited.contains(&current_object_str) {
                    visited.insert(current_object_str.clone());

                    let current_tree_hash = create_tree(&current_object_path, &mindless_root, tracked_files);
                    tree_hashes.insert(current_tree_hash, current_object_str);
                }
            } else {
                // TODO: Only create blobs for objects that have changed

                if let Some(blob_hash) = create_blob(&file, mindless_root) {
                    let file_name = file.file_name()
                        .expect("Something went wrong while getting filename")
                        .to_str()
                        .expect("Something went wrong while converting to a string")
                        .to_owned();

                    blob_hashes.insert(blob_hash, file_name);
                } else {
                    println!("There was an error commiting file.");
                }

                println!("Creating blob of: {}", current_object.as_os_str().to_str().expect("bleh"));
            }
        }
    }

    let mut content = String::new();

    for (blob_hash, file_name) in blob_hashes {
        let current_line = format!("{} {} {}\n", BLOB_OBJECT_TYPE, blob_hash, file_name);
        content.push_str(&current_line)
    }

    for (tree_hash, file_name) in tree_hashes {
        let current_line = format!("{} {} {}\n", TREE_OBJECT_TYPE, tree_hash, file_name);
        content.push_str(&current_line)
    }

    let content_bytes = content.as_bytes();

    let header = format!("{} {}\0", TREE_OBJECT_TYPE, content.len());
    let mut complete_bytes = header.into_bytes();
    complete_bytes.extend(content_bytes);

    return create_object(&complete_bytes, &mindless_root);
}
