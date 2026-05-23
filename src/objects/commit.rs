use std::{fs::{read_to_string, write}, path::PathBuf};

use crate::{
    objects::{object::{create_object, get_object}, tree::create_tree},
    util::{constants::{COMMIT_OBJECT_TYPE, HEAD_FILE_NAME, MINDLESS_DIR_NAME, REFS_DIR_NAME}, mindless::get_head, output::print_project_not_found, workspace::{get_nevermind_patterns, get_root, get_tracked_files}}
};


struct CommitBuilder {
    tree: Option<String>,
    parent: Option<String>,
    message: String,
}


pub struct Commit {
    pub tree: String,
    pub parent: Option<String>,
    pub message: String,
    pub hash: String,
}


pub fn create_commit(message: &str) {
    let Some(mindless_root) = get_root() else {
        print_project_not_found();
        return;
    };

    let nevermind_patterns = get_nevermind_patterns(&mindless_root);
    let tracked_files = get_tracked_files(None, &nevermind_patterns, &mindless_root);
    let tree = create_tree(&mindless_root, &mindless_root, &tracked_files);

    // TODO: Add timestamp
    let mut content = String::from(format!("tree {}\n", tree));

    if let Some(head_hash) = get_head(&mindless_root) {
        content.push_str(&format!("parent {}\n", head_hash));
    }

    content.push_str(&format!("\n{}", message));

    let content_bytes = content.as_bytes();

    let header = format!("{} {}\0", COMMIT_OBJECT_TYPE, content.len());
    let mut complete_bytes = header.into_bytes();
    complete_bytes.extend(content_bytes);

    let commit_hash = create_object(&complete_bytes, &mindless_root);

    let head_file = mindless_root
        .join(MINDLESS_DIR_NAME)
        .join(HEAD_FILE_NAME);
    let head_content = read_to_string(head_file).expect("Something went wrong while reading HEAD");
    let tokens: Vec<&str> = head_content.split(" ").collect();

    if tokens.len() != 2 {
        println!("Something went wrong while committing to head");
        return;
    }

    let ref_type = tokens[0];
    let name = tokens[1];
    let ref_folder = format!("{}s", ref_type);

    let ref_file = mindless_root
        .join(MINDLESS_DIR_NAME)
        .join(REFS_DIR_NAME)
        .join(ref_folder)
        .join(name);

    write(ref_file, commit_hash).expect("Something went wrong while committing hash");

    println!("Changes to workspace saved.");

    // TODO: Show additions/deletions
}


pub fn get_commit(mindless_root: &PathBuf, commit_hash: &str) -> Option<Commit> {
    let commit_contents_option = get_object(mindless_root, commit_hash, true);

    if commit_contents_option == None {
        return None;
    }

    let commit_contents = commit_contents_option.expect("Something went wrong while getting commit hash");
    let mut commit_builder = CommitBuilder{ tree: None, parent: None, message: String::new() };
    let mut newline_found = false;

    for line in commit_contents.lines() {
        if newline_found {
            commit_builder.message.push_str(line);
            continue;
        }

        if line.starts_with("tree") && let Some(tree) = line.strip_prefix("tree ") {
            commit_builder.tree = Some(tree.to_string());
        } else if line.starts_with("parent") && let Some(parent) = line.strip_prefix("parent ") {
            commit_builder.parent = Some(parent.to_string());
        } else if line.trim() == "" {
            newline_found = true;
        }
    }

    if commit_builder.tree == None {
        return None;
    }

    return Some(Commit {
        tree: commit_builder.tree.expect("tree was unexpectedly null"),
        parent: commit_builder.parent,
        message: commit_builder.message,
        hash: commit_hash.to_string()
    });
}
