use std::{fs::read_to_string, path::PathBuf, process};

use crate::util::{
    constants::{EMPTY_BRANCH, HEAD_FILE_NAME, MINDLESS_DIR_NAME, REFS_DIR_NAME},
    output::print_error_reading_head,
};

pub fn get_head_hash(mindless_root: &PathBuf) -> Option<String> {
    let head_file = mindless_root.join(MINDLESS_DIR_NAME).join(HEAD_FILE_NAME);
    let head_content = read_to_string(head_file).expect("Something went wrong while reading HEAD");
    let tokens: Vec<&str> = head_content.split(" ").collect();

    if tokens.len() != 2 {
        return None;
    }

    let ref_type = tokens[0];
    let name = tokens[1];
    let ref_folder = format!("{}s", ref_type);

    let ref_file = mindless_root
        .join(MINDLESS_DIR_NAME)
        .join(REFS_DIR_NAME)
        .join(ref_folder)
        .join(name);

    return match read_to_string(ref_file) {
        Ok(head_hash) => {
            if head_hash == EMPTY_BRANCH {
                None
            } else {
                Some(head_hash)
            }
        }
        Err(_e) => {
            print_error_reading_head();
            process::exit(1);
        }
    };
}
