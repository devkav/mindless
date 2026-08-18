use crate::{
    objects::{
        commit::get_head_commit_or_exit,
        tree::{Tree, TreeNode, create_tree_object},
    },
    util::{
        mindless::get_head_hash,
        workspace::{get_nevermind_patterns, get_root_or_exit, get_tracked_files},
    },
};

pub fn test_print_tree(tree: Tree, indent: &str) {
    for (_, child_node) in tree.children {
        match child_node {
            TreeNode::SubTree { name, hash, tree } => {
                let child_indent = format!("{indent} ");

                println!("{indent}{name} {hash}");
                test_print_tree(tree, &child_indent);
            }
            TreeNode::Blob { name, hash } => {
                println!("{indent}{name} {hash}");
            }
        }
    }
}

pub fn diff() {
    // TODO: Add diffs between 2 commits
    // TODO: Add diffs for a given file

    let mindless_root = get_root_or_exit();

    if let Some(head_hash) = get_head_hash(&mindless_root) {
        let head_commit = get_head_commit_or_exit(&mindless_root, &head_hash);

        let nevermind_patterns = get_nevermind_patterns(&mindless_root);
        let tracked_files = get_tracked_files(None, &nevermind_patterns, &mindless_root);

        let (hash, tree) = create_tree_object(&mindless_root, &mindless_root, &tracked_files);
        test_print_tree(tree, "");
        // TODO: Need to refactor to make create_tree return a Tree object, instead of a hash.
        // It should create a tree object without creating a tree file
        //
        // let change_report = get_tree_diff(&mindless_root, &head_commit.tree, &tree, "");
    } else {
        // Display diff of all changes
    }
}
