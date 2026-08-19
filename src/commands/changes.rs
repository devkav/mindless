use colored::Colorize;
use crate::{objects::{commit::get_head_commit_or_exit, tree::{create_tree, get_tree_diff}}, util::{mindless::get_head_hash, workspace::{get_nevermind_patterns, get_root_or_exit, get_tracked_files}}};

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

        println!("analyzing diff");
        if let Some(change_report) = get_tree_diff(&mindless_root, &tree_hash, &current_tree, "") {
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
        // TODO: Need to create a fake tree without adding it to history
        // let change_report = get_tree_diff(&mindless_root, &head_commit.tree, &tree, "");
    } else {
        // Display diff of all changes
    }
}
