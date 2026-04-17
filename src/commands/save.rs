use crate::util::{blob::compress_file, files::{get_nevermind_patterns, get_root, get_tracked_files}};


pub fn save(message: String) {
    println!("Saving directory state with message: {}", message);
    
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
