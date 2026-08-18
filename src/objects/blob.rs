use std::{
    fs::{self},
    path::PathBuf,
};

use crate::objects::object::{Object, ObjectType, create_object_file};

pub fn create_blob_object(file_path: &PathBuf) -> Option<Object> {
    let input = fs::read(file_path);

    match input {
        Ok(content) => {
            return Some(Object {
                object_type: ObjectType::BLOB,
                content,
            });
        }
        Err(_e) => {
            // TODO
            return None;
        }
    }
}

pub fn create_blob_file(file_path: &PathBuf, mindless_root: &PathBuf) -> Option<String> {
    return match create_blob_object(file_path) {
        Some(object) => Some(create_object_file(&object, mindless_root)),
        None => None,
    };
}
