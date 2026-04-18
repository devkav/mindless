use std::{env, fs::{create_dir, write}};

use crate::util::constants::{
    EMPTY_BRANCH, HEAD_FILE_NAME, HEAD_REF_TYPE, HEADS_DIR_NAME, MAIN_BRANCH_NAME, MINDLESS_DIR_NAME, OBJECTS_DIR_NAME, REFS_DIR_NAME
};


pub fn init() {
    let working_directory = env::current_dir().expect("Couldn't find working directory");
    let mindless_directory = working_directory.join(MINDLESS_DIR_NAME);

    if mindless_directory.exists() {
        println!("mindless project already exists.");
        return;
    }

    let object_dir = mindless_directory.join(OBJECTS_DIR_NAME);
    let refs_dir = mindless_directory.join(REFS_DIR_NAME);
    let heads_ref_dir = refs_dir.join(HEADS_DIR_NAME);
    let head_file = mindless_directory.join(HEAD_FILE_NAME);
    let main_head_file = heads_ref_dir.join(MAIN_BRANCH_NAME);

    let head_file_content = format!("{} {}", HEAD_REF_TYPE, MAIN_BRANCH_NAME);

    match create_dir(mindless_directory) {
        Ok(_) => {
            create_dir(object_dir).expect("Something went wrong while creating objects directory");
            create_dir(refs_dir).expect("Something went wrong while creating refs directory");
            create_dir(heads_ref_dir).expect("Something went wrong while creating heads directory");

            write(head_file, head_file_content).expect("Something went wrong while create HEAD file");
            write(main_head_file, EMPTY_BRANCH).expect("Something went wrong while create main head file");

            println!("mindless project initialized.")
        },
        Err(_) => {
            println!("There was an error creating a project.")
        }
    }
}
