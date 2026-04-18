use std::{fs::{self, create_dir}, path::PathBuf};
use zlib_rs::{DeflateConfig, compress_bound, compress_slice};
use sha2::{Digest, Sha256};

use crate::util::{constants::{MINDLESS_DIR_NAME, OBJECTS_DIR_NAME}};


pub fn get_hash(content: &[u8]) -> String {
    let hash = Sha256::digest(content);
    let hash_hex = hex::encode(hash);

    return hash_hex;
}


pub fn create_object(content_b: &Vec<u8>, mindless_root: &PathBuf) -> String {
    let mut compressed_buf = vec![0u8; compress_bound(content_b.len())];
    let (compressed, _) = compress_slice(&mut compressed_buf, &content_b, DeflateConfig::default());

    let complete_hash = get_hash(&content_b);
    let object_dir_name = &complete_hash[0..2];
    let file_name = &complete_hash[2..];

    let object_dir_path = mindless_root
        .join(MINDLESS_DIR_NAME)
        .join(OBJECTS_DIR_NAME)
        .join(object_dir_name);

    if !object_dir_path.exists() {
        create_dir(&object_dir_path).expect("Something went wrong while creating object directory.");
    }

    let output_path = object_dir_path.join(file_name);
    fs::write(output_path, &compressed).expect("Could not write to file");

    // TODO: Add some pretty printing

    return complete_hash;
}
