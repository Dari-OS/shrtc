pub mod command;
mod file;


use std::process::exit;
use serde_json;
use command::Command;


pub fn add(command: &Command) {
    let mut file_content = all();
    file_content.push(command.clone());
    set(file_content);
}

pub fn remove(name: &str) {
    let mut file_content = all();
    'removal: for  (index, command) in file_content.iter_mut().enumerate() {
        if command.name() == name {
            file_content.remove(index);
            break 'removal; //for educational purposes
        }
    }
    set(file_content);
}

pub fn all() -> Vec<Command> {
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

pub fn set(commands: Vec<Command>) {
    let json = serde_json::to_string_pretty(&commands).unwrap(); //Should never fail
    file::write_data(&json).unwrap_or_else(|e| {
        println!("Could not write to file {}", file::DATA_NAME);
        println!("{}", e);
        exit(1)
    });
}