use crate::util::blob::compress_blob;

pub fn save(message: String) {
    // TODO: Respect .nevermind (gitignore equivalent)
    // TODO: Should not need to specify a path. No staging environment.
    println!("Saving directory state with message: {}", message);
    
    compress_blob(".".to_string());
}
