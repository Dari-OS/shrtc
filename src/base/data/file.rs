use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde_json;
use super::shortcut::Shortcut;

pub const DATA_NAME: &str = "shrtc.data";
const FOLDER_NAME: &str = ".shrtc";
const SCRIPT_FOLDER_NAME: &str = "scripts"; //TODO Add support to store scripts in the config



/// This initializes the file structure for **shrtc**
fn initialize() {
    let data_path = format!("{}/{}", &FOLDER_NAME, &DATA_NAME);
    let script_path = format!("{}/{}", &FOLDER_NAME, &SCRIPT_FOLDER_NAME);
    if !Path::new(&FOLDER_NAME).exists() {
        if let Err(e)= fs::create_dir(&FOLDER_NAME) {
            panic!("Could not create directory ({}) for shrtc:\n{}",&FOLDER_NAME, e);
        }
    }

    if !Path::new(&data_path).exists() {
        match fs::File::create(&data_path) {
            Ok(mut file) => {
                file.write_all(default_to_json().as_bytes()).unwrap_or_else(|err| {
                    panic!("Could not write content into {}:\n{}", &data_path, err);
                });

            }
            Err(err) => {
                panic!("Could not create directory for shrtc:\n{:?}", err);

            }
        }
    }

    if !Path::new(&script_path).exists() {
        if let Err(e)= fs::create_dir(&script_path) {
            panic!("Could not create directory ({}) for shrtc scripts:\n{}",&script_path, e);
        }
    }
}

fn default_to_json() -> String{
    let default_commands: Vec<Shortcut> = vec![
        Shortcut::new("add", "shrtc-add", true),
        Shortcut::new("rm", "shrtc-rm", true),
        Shortcut::new("ls", "shrtc-ls", true),
        Shortcut::new("get", "shrtc-get", true),
    ];

    serde_json::to_string_pretty(&default_commands).unwrap() //This should never throw an error.
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