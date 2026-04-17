use std::{fs::{self}, path::PathBuf};

use crate::{
    objects::object::create_object,
    util::constants::BLOB_OBJECT_TYPE
};


pub fn create_blob(file_path: &PathBuf, mindless_root: &PathBuf) {
    let input = fs::read(file_path);

    match input {
        Ok(file_bytes) => {
            let header = format!("{} {}\0", BLOB_OBJECT_TYPE, file_bytes.len());
            let mut blob = header.into_bytes();
            blob.extend(&file_bytes);

            create_object(&blob, mindless_root);
        },
        Err(_e) => {
            // TODO
        }
    }
}

