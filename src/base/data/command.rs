use std::io;
use std::process::Output;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        return &mut self.name;
    }

    pub fn command(&mut self) -> &mut String {
        return &mut self.name;
    }

    pub fn default(&mut self) -> &mut bool {
        return &mut self.default_command;
    }

    pub fn execute(&self, args: &str) -> io::Result<Output> {
        let (shell, shell_arg) = match std::env::consts::OS {
            "windows" => ("cmd", "/c"),
            _unix => ("sh", "-c"),
        };

        std::process::Command::new(shell)
            .arg(shell_arg)
            .arg(&self.name)
            .arg(args)
            .output()
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }

    pub fn from_json(jsonified_string: &String) -> serde_json::Result<Self> {
        serde_json::from_str(jsonified_string)
    }
}

