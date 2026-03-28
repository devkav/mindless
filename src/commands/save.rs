use crate::util::{blob::compress_blob, files::get_tracked_files};

pub fn save(message: String) {
    // TODO: Respect .nevermind (gitignore equivalent)
    // TODO: Should not need to specify a path. No staging environment.
    println!("Saving directory state with message: {}", message);
    
    // compress_blob(".".to_string());

    let tracked_files = get_tracked_files(None).unwrap();

    for file in tracked_files {
        println!("{}", file.display());
    }
}
