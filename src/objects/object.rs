use sha2::{Digest, Sha256};
use std::{
    fs::{self, create_dir},
    path::PathBuf,
};
use zlib_rs::{DeflateConfig, compress_bound, compress_slice};

use crate::util::{
    constants::{MINDLESS_DIR_NAME, OBJECTS_DIR_NAME},
    files::decompress_file,
};

pub enum ObjectType {
    COMMIT,
    TREE,
    BLOB,
}

impl ObjectType {
    pub fn as_str(&self) -> String {
        match self {
            ObjectType::COMMIT => String::from("commit"),
            ObjectType::TREE => String::from("tree"),
            ObjectType::BLOB => String::from("blob"),
        }
    }
}

pub struct Object {
    pub object_type: ObjectType,
    pub content: Vec<u8>,
}

impl Object {
    pub fn get_hash(&self) -> String {
        let hash = Sha256::digest(self.get_bytes());
        let hash_hex = hex::encode(hash);

        return hash_hex;
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let header = format!("{} {}\0", self.object_type.as_str(), self.content.len());
        let mut complete_bytes = header.into_bytes();
        complete_bytes.extend(&self.content);

        return complete_bytes;
    }
}

pub fn create_object_file(object: &Object, mindless_root: &PathBuf) -> String {
    let object_bytes = object.get_bytes();
    let object_hash = object.get_hash();

    let mut compressed_buf = vec![0u8; compress_bound(object_bytes.len())];
    let (compressed, _) =
        compress_slice(&mut compressed_buf, &object_bytes, DeflateConfig::default());

    let object_dir_name = &object_hash[0..2];
    let file_name = &object_hash[2..];

    let object_dir_path = mindless_root
        .join(MINDLESS_DIR_NAME)
        .join(OBJECTS_DIR_NAME)
        .join(object_dir_name);

    if !object_dir_path.exists() {
        create_dir(&object_dir_path)
            .expect("Something went wrong while creating object directory.");
    }

    let output_path = object_dir_path.join(file_name);
    fs::write(output_path, &compressed).expect("Could not write to file");

    // TODO: Add some pretty printing

    return object_hash;
}

pub fn object_exists(mindless_root: &PathBuf, object_hash: &str) -> bool {
    let object_dir_name = &object_hash[0..2];
    let object_file_name = &object_hash[2..];

    let object_file = mindless_root
        .join(MINDLESS_DIR_NAME)
        .join(OBJECTS_DIR_NAME)
        .join(object_dir_name)
        .join(object_file_name);

    return object_file.exists();
}

pub fn get_object(mindless_root: &PathBuf, object_hash: &str, remove_type: bool) -> Option<String> {
    let object_dir_name = &object_hash[0..2];
    let object_file_name = &object_hash[2..];

    let object_file = mindless_root
        .join(MINDLESS_DIR_NAME)
        .join(OBJECTS_DIR_NAME)
        .join(object_dir_name)
        .join(object_file_name);

    if !object_file.exists() || !object_file.is_file() {
        return None;
    }

    let object_file_str = object_file
        .to_str()
        .expect("Error converting path to string")
        .to_string();

    let file_contents_option = Some(decompress_file(&object_file_str));

    return match file_contents_option {
        Some(file_contents) => {
            if remove_type && file_contents.contains("\0") {
                let file_str_tokens = file_contents
                    .split_once('\0')
                    .expect("Something went wrong while removing object type");

                Some(file_str_tokens.1.to_string())
            } else {
                Some(file_contents)
            }
        }
        None => None,
    };
}
