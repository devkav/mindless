use crate::{
    objects::{
        commit::get_head_commit_or_exit,
        tree::{create_tree, get_tree_diff},
    },
    util::{
        mindless::get_head_hash,
        workspace::{get_nevermind_patterns, get_root_or_exit, get_tracked_files},
    },
};
use colored::Colorize;

pub fn changes() {
    // TODO: Add diffs between 2 commits
    // TODO: Add diffs for a given file

    let mindless_root = get_root_or_exit();

    if let Some(head_hash) = get_head_hash(&mindless_root) {
        let head_commit = get_head_commit_or_exit(&mindless_root, &head_hash);
        let tree_hash = head_commit.tree;

        let nevermind_patterns = get_nevermind_patterns(&mindless_root);
        let tracked_files = get_tracked_files(None, &nevermind_patterns, &mindless_root);
        let current_tree = create_tree(&mindless_root, &mindless_root, &tracked_files);

        if let Some(change_report) = get_tree_diff(&mindless_root, &tree_hash, &current_tree, "") {
            if change_report.is_empty() {
                println!("No changes have been made.");
            } else {
                println!("{change_report}");
            }
        } else {
            println!(
                "{}",
                "There was an unexpected error while getting diff.".red()
            )
        }
    } else {
        // TODO: Display diff of all changes
        println!("New repo, everything is new. This is TODO.");
    }
}
