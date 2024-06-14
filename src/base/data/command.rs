use std::io;
use serde::{Serialize, Deserializer};

#[derive(Debug, )]
pub struct Command {
    name: String,
    command: String,
    default_command: bool,
}


impl Command {
    pub fn new(name: &str, command: &str, is_default: bool) -> Command {
        Command {
            name: String::from(name),
            command: String::from(command),
            default_command: is_default
        }
    }

    pub fn name(&mut self) -> &mut String {
        return  &mut self.name;
    }

    pub fn command(&mut self) -> &mut String {
        return &mut self.name;
    }

    pub fn default(&mut self) -> &mut bool {
        return &mut self.default_command;
    }

    pub fn execute(&self, args: &str) -> Result<(), io::Error>{
        let (shell, shell_arg) = match std::env::consts::OS {
            "windows" => ("cmd","/c"),
            unix => ("sh", "-c"),
        };

        match std::process::Command::new(shell)
            .arg(shell_arg)
            .arg(&self.name)
            .arg(args)
            .output() {
            Ok(_) => {Ok(())}
            Err(e) => {Err(e)}
        }

    }

}