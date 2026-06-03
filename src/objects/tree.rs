use std::{collections::{HashMap, HashSet}, fmt, path::PathBuf};

use colored::Colorize;
use similar::{ChangeTag, TextDiff};

use crate::{objects::{blob::create_blob, object::{create_object, get_object}}, util::{constants::{BLOB_OBJECT_TYPE, TREE_OBJECT_TYPE}, output::print_error_reading_tree}};

pub enum TreeNode {
    Blob {
        name: String,
        hash: String
    },
    SubTree {
        name: String,
        hash: String,
        tree: Tree
    }
}

impl TreeNode {
    pub fn get_hash(&self) -> &str {
        match self {
            TreeNode::Blob { hash, .. } => {
                hash
            },
            TreeNode::SubTree { hash, .. } => {
                hash
            }
        }
    }
}

pub struct Tree {
    pub children: HashMap<String, TreeNode>
}

// TODO: Implement Moves
pub enum ChangeMode {
    CREATION,
    DELETION,
    CHANGE
}

pub struct Change {
    pub additions: usize,
    pub deletions: usize,
    pub mode: ChangeMode,
    pub filename: String
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{} {}",
            format!("+{}", self.additions).green().bold(),
            format!("-{}", self.additions).red().bold(),
            self.filename
        )
    }
}

pub struct ChangeReport {
    pub new_files: Vec<Change>,
    pub deleted_files: Vec<Change>,
    pub changed_files: Vec<Change>
}


pub fn create_tree(path_prefix: &PathBuf, mindless_root: &PathBuf, tracked_files: &Vec<PathBuf>) -> String {
    let mut visited = HashSet::new();
    let mut tree_hashes: HashMap<String, String> = HashMap::new();
    let mut blob_hashes: HashMap<String, String> = HashMap::new();
    
    for file in tracked_files {
        if !file.starts_with(path_prefix) {
            continue;
        }

        let relative_path = file.strip_prefix(path_prefix).expect("Something went wrong while stripping root prefix");
        let current_object_option = relative_path.components().next();

        if let Some(current_object) = current_object_option {
            let current_object_path = path_prefix.join(current_object);
            let current_object_str = current_object
                .as_os_str()
                .to_str()
                .expect("Something went wrong while converting to a string")
                .to_owned();

            if current_object_path.is_dir() {
                if !visited.contains(&current_object_str) {
                    visited.insert(current_object_str.clone());

                    let current_tree_hash = create_tree(&current_object_path, &mindless_root, tracked_files);
                    tree_hashes.insert(current_object_str, current_tree_hash);
                }
            } else {
                // TODO: Only create blobs for objects that have changed
                // If the file has not changed, the blob will be the same!
                // So techinically we don't need to check when the file was last changed
                // However, I think we should to avoid any redundant compressions

                if let Some(blob_hash) = create_blob(&file, mindless_root) {
                    let file_name = file.file_name()
                        .expect("Something went wrong while getting filename")
                        .to_str()
                        .expect("Something went wrong while converting to a string")
                        .to_owned();

                    blob_hashes.insert(file_name, blob_hash);
                } else {
                    println!("There was an error commiting file.");
                }
            }
        }
    }

    let mut content = String::new();

    for (file_name, blob_hash) in blob_hashes {
        let current_line = format!("{} {} {}\n", BLOB_OBJECT_TYPE, blob_hash, file_name);
        content.push_str(&current_line)
    }

    for (file_name, tree_hash) in tree_hashes {
        let current_line = format!("{} {} {}\n", TREE_OBJECT_TYPE, tree_hash, file_name);
        content.push_str(&current_line)
    }

    let content_bytes = content.as_bytes();

    let header = format!("{} {}\0", TREE_OBJECT_TYPE, content.len());
    let mut complete_bytes = header.into_bytes();
    complete_bytes.extend(content_bytes);

    return create_object(&complete_bytes, &mindless_root);
}


pub fn get_tree(mindless_root: &PathBuf, hash: &str) -> Option<Tree> {
    if let Some(object) = get_object(mindless_root, hash, true) {
        let mut children: HashMap<String, TreeNode> = HashMap::new();

        for line in object.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();

            if tokens.len() != 3 {
                println!("{}", format!("Malformed tree {}. Skipping...", hash).yellow());
                continue;
            }

            let child_type = tokens[0];
            let child_hash = tokens[1];
            let child_name = tokens[2];

            if child_type != TREE_OBJECT_TYPE && child_type != BLOB_OBJECT_TYPE {
                println!("{}", format!("Invalid type {} found in tree. Skipping...", child_type).yellow());
                continue;
            }

            if child_type == TREE_OBJECT_TYPE {
                if let Some(child_tree) = get_tree(&mindless_root, child_hash) {
                    children.insert(child_name.to_string(), TreeNode::SubTree {
                        name: child_name.to_string(),
                        hash: child_hash.to_string(),
                        tree: child_tree
                    });
                }
            } else {
                children.insert(child_name.to_string(), TreeNode::Blob {
                    name: child_name.to_string(),
                    hash: child_hash.to_string()
                });
            };

        }

        let tree = Tree { children };
        return Some(tree);
    } else {
        return None
    }
}


pub fn get_tree_diff(mindless_root: &PathBuf, parent: &str, child: &str, path_from_root: &str) -> Option<ChangeReport> {
    let mut new_files: Vec<Change> = Vec::new();
    let mut deleted_files: Vec<Change> = Vec::new();
    let mut changed_files: Vec<Change> = Vec::new();

    // TODO: Need to cover both new files and deleted files
    // TODO: Make work for initial commits

    if let Some(parent_object) = get_tree(mindless_root, parent) && 
        let Some(child_object) = get_tree(mindless_root, child) {
            for (name, tree_node) in child_object.children {
                let current_hash = tree_node.get_hash();

                match parent_object.children.get(&name) {
                    Some(prev_hash_object) => {
                        let prev_hash = prev_hash_object.get_hash();

                        if prev_hash != current_hash {
                            match tree_node {
                                TreeNode::SubTree { .. } => {
                                    let path_to_tree = format!("{}{}/", path_from_root, name);

                                    if let Some(nested_change_report) = get_tree_diff(mindless_root, prev_hash, &current_hash, &path_to_tree) {
                                        new_files.extend(nested_change_report.new_files);
                                        deleted_files.extend(nested_change_report.deleted_files);
                                        changed_files.extend(nested_change_report.changed_files);
                                    }
                                }
                                TreeNode::Blob { .. } => {
                                    let path_to_blob = format!("{}{}", path_from_root, name);

                                    if let Some(prev_blob) = get_object(mindless_root, prev_hash, true) &&
                                        let Some(blob) = get_object(mindless_root, current_hash, true) {
                                            let diff = TextDiff::from_lines(&prev_blob, &blob);
                                            let mut additions = 0;
                                            let mut deletions = 0;

                                            for change in diff.iter_all_changes() {
                                                match change.tag() {
                                                    ChangeTag::Equal => {},
                                                    ChangeTag::Insert => additions += 1,
                                                    ChangeTag::Delete => deletions += 1
                                                }
                                            }

                                            changed_files.push(Change {
                                                additions: additions,
                                                deletions: deletions,
                                                mode: ChangeMode::CHANGE,
                                                filename: path_to_blob
                                            });
                                    }
                                }
                            }
                        }
                    },
                    None => {
                        match &tree_node {
                            TreeNode::SubTree { .. } => {
                                let mut queue: Vec<(TreeNode, String)> = vec![(tree_node, path_from_root.to_string())];

                                while let Some((current, path_to_parent)) = queue.pop() {
                                    match current {
                                        TreeNode::SubTree { name, tree, .. } => {
                                            let path_to_tree = format!("{}{}/", path_to_parent, name);

                                            for (_child_name, child) in tree.children {
                                                queue.push((child, path_to_tree.to_string()));
                                            }
                                        }
                                        TreeNode::Blob { name, hash } => {
                                            let path_to_blob = format!("{}{}", path_to_parent, name);

                                            if let Some(blob_content) = get_object(mindless_root, &hash, true) {
                                                new_files.push(Change {
                                                    additions: blob_content.lines().count(),
                                                    deletions: 0,
                                                    mode: ChangeMode::CREATION,
                                                    filename: path_to_blob
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            TreeNode::Blob { name, hash } => {
                                let path_to_blob = format!("{}{}", path_from_root, name);

                                if let Some(blob_content) = get_object(mindless_root, hash, true) {
                                    new_files.push(Change {
                                        additions: blob_content.lines().count(),
                                        deletions: 0,
                                        mode: ChangeMode::CREATION,
                                        filename: path_to_blob
                                    });
                                }
                            }
                        }
                    }
                };
            }
    } else {
        print_error_reading_tree();
        return None;
    }

    let change_report = ChangeReport {
        new_files,
        deleted_files,
        changed_files
    };

    return Some(change_report);
}
