use crate::{objects::commit::get_commit, util::{mindless::get_head_hash, output::{print_no_history}, workspace::{get_root_or_exit}}};


pub fn history() {
    let mindless_root = get_root_or_exit();
    let head_hash = get_head_hash(&mindless_root);
    let mut current_hash_option = head_hash;

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
