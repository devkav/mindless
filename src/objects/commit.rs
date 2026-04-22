use std::fs::{read_to_string, write};

use crate::{
    objects::{object::create_object, tree::create_tree},
    util::{constants::{COMMIT_OBJECT_TYPE, HEAD_FILE_NAME, MINDLESS_DIR_NAME, REFS_DIR_NAME}, mindless::get_head, workspace::{get_nevermind_patterns, get_root, get_tracked_files}}
};


pub fn create_commit(message: &str) {
    let Some(mindless_root) = get_root() else {
        println!("No mindless project found.");
        return;
    };

    let nevermind_patterns = get_nevermind_patterns(&mindless_root);
    let tracked_files = get_tracked_files(None, &nevermind_patterns, &mindless_root);
    let tree = create_tree(&mindless_root, &mindless_root, &tracked_files);

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
