pub mod command;
mod file;


use std::io;
use serde_json;
use command::Command;

pub fn add(command: Command) -> Result<(), io::Error>{
    let deserialized: Vec<Command> = serde_json::from_str(file::read_data()?.as_str())?;
}

pub fn remove(name: &str) {

}

pub fn all() -> Vec<Command> {

}

pub fn set(commands: Vec<Command>) {


}