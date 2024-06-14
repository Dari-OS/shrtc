const DATA_NAME: &str = "shrtc.data";
const FOLDER_NAME: &str = ".shrtc";
const DATA_PATH: String = format!("{}/{}", FOLDER_NAME, DATA_NAME);

use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::Path;


/// This initializes the file structure for **shrtc**
fn initialize() {
    if !Path::new(FOLDER_NAME).exists() {
        if let Err(e)= fs::create_dir(FOLDER_NAME) {
            panic!("Could not create directory ({}) for shrtc:\n{}",FOLDER_NAME, e);
        }
    }

    if !Path::new(&DATA_PATH).exists() {
        match fs::File::create(&DATA_PATH) {
            Ok(mut file) => {
                file.write_all(b"[]").unwrap_or_else(|err| {
                    panic!("Could not write content into {}:\n{}", &DATA_PATH, err);
                });

            }
            Err(err) => {
                panic!("Could not create directory for shrtc:\n{:?}", err);

            }
        }
    }
}

pub fn write_data(content: &str) -> io::Result<()> {
    initialize();
    File::open(&DATA_PATH).unwrap()?.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_data() -> Result<String, io::Error> {
    Ok(fs::read_to_string(DATA_PATH)?)
}