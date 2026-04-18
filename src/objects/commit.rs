use crate::{
    objects::{object::create_object, tree::create_tree},
    util::{constants::COMMIT_OBJECT_TYPE, workspace::{get_nevermind_patterns, get_root, get_tracked_files}}
};


pub fn create_commit(message: &str) {
    let Some(mindless_root) = get_root() else {
        println!("No mindless project found.");
        return;
    };

    let nevermind_patterns = get_nevermind_patterns(&mindless_root);
    let tracked_files = get_tracked_files(None, &nevermind_patterns, &mindless_root);
    let tree = create_tree(&mindless_root, &mindless_root, &tracked_files);

    // TODO: Add parent!

    let content = format!(
        "tree {}\n\
        parent<PARENT>\n\
        \n\
        {}",
        tree, message
    );

    let content_bytes = content.as_bytes();

    let header = format!("{} {}\0", COMMIT_OBJECT_TYPE, content.len());
    let mut complete_bytes = header.into_bytes();
    complete_bytes.extend(content_bytes);

    let commit_hash = create_object(&complete_bytes, &mindless_root);
    println!("{}", commit_hash);
}
