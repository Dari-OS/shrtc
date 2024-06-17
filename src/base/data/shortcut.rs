use std::io;
use std::process::Output;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    name: String,
    command: String,
    default_command: bool,
}


impl Shortcut {
    pub fn new(name: &str, command: &str, is_default: bool) -> Shortcut {
        Shortcut {
            name: String::from(name),
            command: String::from(command),
            default_command: is_default
        }
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn command(&mut self) -> &mut String {
        return &mut self.command;
    }

    pub fn default(&self) -> &bool {
        return &self.default_command;
    }

    pub fn execute(&self, args: &str) -> io::Result<Output> {
        let (shell, shell_arg) = match std::env::consts::OS {
            "windows" => ("cmd", "/c"),
            _unix => ("sh", "-c"),

        };
        let combined_command = format!("{} {}", &self.command, args);
        std::process::Command::new(shell)
            .arg(shell_arg)
            .arg(combined_command)
            .output()
    }
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }

    pub fn from_json(jsonified_string: &String) -> serde_json::Result<Self> {
        serde_json::from_str(jsonified_string)
    }
}

