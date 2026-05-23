use crate::{objects::commit::get_commit, util::{mindless::get_head, output::{print_no_history, print_project_not_found}, workspace::get_root}};


pub fn history() {
    let Some(mindless_root) = get_root() else {
        print_project_not_found();
        return;
    };

    let head = get_head(&mindless_root);
    let mut current_hash_option = head;

    if current_hash_option == None {
        print_no_history();
    }

    while let Some(current_hash) = current_hash_option {
        let current_commit_option = get_commit(&mindless_root, &current_hash);

        if let Some(current_commit) = current_commit_option {
            current_hash_option = current_commit.parent;
            println!("Hash: {}, message: {}", current_commit.hash, current_commit.message);
        } else {
            current_hash_option = None;
        }
    }
}
