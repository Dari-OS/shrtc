pub mod shortcut;
mod file;


use std::process::exit;
use serde_json;
use shortcut::Shortcut;

///
/// Adds a *shortcut* to the file.
/// Returns `true` if the *shortcut* was successfully added to the file.
/// Returns `false` if the *shortcut* is already declared
///
pub fn add(command: &Shortcut) -> bool{
    let mut file_content = all();

    for temp_Command in &file_content { //Checks if the name is already being used
        if temp_Command.name() == command.name() {
        return false;
        }
    };

    file_content.push(command.clone());
    set(file_content);
    true
}

///
/// Removes a *shortcut* from the file.
/// Returns `true` if the *shortcut* was successfully removed from the file.
/// Returns `false` if the *shortcut* was not declared
///
pub fn remove(name: &str) -> bool {
    let mut file_content = all();
    let mut removed = false;
    'removal: for  (index, command) in file_content.iter_mut().enumerate() {
        if command.name() == name {
            file_content.remove(index);
            removed = true;
            break 'removal; //for educational purposes
        }
    }
    if removed {
      set(file_content);
    }
    removed

}

pub fn all() -> Vec<Shortcut> {
    let content = file::read_data().unwrap_or_else(|e| {
            println!("Could not read from file {}", file::DATA_NAME);
            println!("{}", e);
            exit(1)
    });

    serde_json::from_str(&content).unwrap_or_else(|e| {
        println!("Could not parse the JSON in file {}", file::DATA_NAME);
        println!("{}", e);
        exit(1)
    })
}

pub fn set(commands: Vec<Shortcut>) {
    let json = serde_json::to_string_pretty(&commands).unwrap(); //Should never fail
    file::write_data(&json).unwrap_or_else(|e| {
        println!("Could not write to file {}", file::DATA_NAME);
        println!("{}", e);
        exit(1)
    });
}