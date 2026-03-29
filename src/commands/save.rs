use crate::util::{blob::compress_file, files::{get_nevermind_patterns, get_root, get_tracked_files}};

pub fn save(message: String) {
    // TODO: Respect .nevermind (gitignore equivalent)
    // TODO: Should not need to specify a path. No staging environment.
    println!("Saving directory state with message: {}", message);
    
    // compress_blob(".".to_string());

    let Some(mindless_root) = get_root() else {
        println!("No mindless project found.");
        return;
    };

    let nevermind_patterns = get_nevermind_patterns(&mindless_root);
    let tracked_files = get_tracked_files(None, &nevermind_patterns, &mindless_root);

    for file in tracked_files {
        compress_file(&file, &mindless_root);
    }
}
