pub const DATA_NAME: &str = "shrtc.data";
const FOLDER_NAME: &str = ".shrtc";

const DEFAULT_COMMANDS: &str = r"[]";

use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::Path;



/// This initializes the file structure for **shrtc**
fn initialize() {
    let data_path = format!("{}/{}", &FOLDER_NAME, &DATA_NAME);
    if !Path::new(&FOLDER_NAME).exists() {
        if let Err(e)= fs::create_dir(&FOLDER_NAME) {
            panic!("Could not create directory ({}) for shrtc:\n{}",&FOLDER_NAME, e);
        }
    }

    if !Path::new(&data_path).exists() {
        match fs::File::create(&data_path) {
            Ok(mut file) => {
                file.write_all(DEFAULT_COMMANDS.as_bytes()).unwrap_or_else(|err| {
                    panic!("Could not write content into {}:\n{}", &data_path, err);
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
    let data_path = format!("{}/{}", &FOLDER_NAME, &DATA_NAME);
    let mut file = File::create(&data_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_data() -> Result<String, io::Error> {
    initialize();
    Ok(fs::read_to_string(format!("{}/{}", &FOLDER_NAME, &DATA_NAME))?)
}