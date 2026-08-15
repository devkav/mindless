use std::{fs::{read_to_string, write}, path::PathBuf, process};

use colored::Colorize;

use crate::{
    objects::{object::{create_object, get_object}, tree::{create_tree, get_tree_diff}}, util::{constants::{COMMIT_OBJECT_TYPE, HEAD_FILE_NAME, MINDLESS_DIR_NAME, REFS_DIR_NAME}, mindless::get_head_hash, output::{print_error_reading_head}, workspace::{get_nevermind_patterns, get_root_or_exit, get_tracked_files}}
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
    let mindless_root = get_root_or_exit();
    let nevermind_patterns = get_nevermind_patterns(&mindless_root);
    let tracked_files = get_tracked_files(None, &nevermind_patterns, &mindless_root);
    let tree = create_tree(&mindless_root, &mindless_root, &tracked_files);

    // TODO: Add timestamp
    let mut content = String::from(format!("tree {}\n", tree));

    if let Some(head_hash) = get_head_hash(&mindless_root) {
        content.push_str(&format!("parent {}\n", head_hash));
        let head_commit = get_head_commit_or_exit(&mindless_root, &head_hash);

        if let Some(change_report) = get_tree_diff(&mindless_root, &head_commit.tree, &tree, "") {
            println!("Changes saved to project.\n");

            if change_report.new_files.len() > 0 {
                println!("{}", "New Files".bold());

                for change in change_report.new_files {
                    println!("  {}", change.to_string());
                }
            }

            if change_report.deleted_files.len() > 0 {
                println!("{}", "Deleted Files".bold());

                for change in change_report.deleted_files {
                    println!("  {}", change.to_string());
                }
            }

            if change_report.changed_files.len() > 0 {
                println!("{}", "Changed Files".bold());

                for change in change_report.changed_files {
                    println!("  {}", change.to_string());
                }
            }
        } else {
            println!("{}", "There was an unexpected error while getting diff.".red())
        }
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
        print_error_reading_head();
        process::exit(1);
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


pub fn get_head_commit_or_exit(mindless_root: &PathBuf, head_hash: &str) -> Commit {
    let Some(head_commit) = get_commit(&mindless_root, head_hash) else {
        print_error_reading_head();
        process::exit(1);
    };

    return head_commit;
}
