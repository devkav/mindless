use crate::{objects::{commit::{get_head_commit_or_exit}}, util::{mindless::get_head_hash, workspace::get_root_or_exit}};

pub fn diff() {
    // TODO: Add diffs between 2 commits
    // TODO: Add diffs for a given file

    let mindless_root = get_root_or_exit();

    if let Some(head_hash) = get_head_hash(&mindless_root) {
        let head_commit = get_head_commit_or_exit(&mindless_root, &head_hash);
        // TODO: Need to create a fake tree without adding it to history
        // let change_report = get_tree_diff(&mindless_root, &head_commit.tree, &tree, "");
    } else {
        // Display diff of all changes
    }
}
