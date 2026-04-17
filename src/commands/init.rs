use std::{env, fs::create_dir};

use crate::util::constants::{MINDLESS_DIR_NAME, OBJECTS_DIR_NAME};


pub fn init() {
    let working_directory = env::current_dir().expect("Couldn't find working directory");
    let mindless_directory = working_directory.join(MINDLESS_DIR_NAME);
    let object_dir = mindless_directory.join(OBJECTS_DIR_NAME);

    if mindless_directory.exists() {
        println!("mindless project already exists.");
        return;
    }

    match create_dir(mindless_directory) {
        Ok(_) => {
            create_dir(object_dir).expect("Something went wrong while creating objects directory");
            println!("mindless project initialized.")
        },
        Err(_) => {
            println!("There was an error creating a project.")
        }
    }
}
