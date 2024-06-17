extern crate base;
use base::{data,data::shortcut::Shortcut};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a name for the shortcut.\
        \nDo \"shrtc help\" to get a list of the default commands and their usage.");
        return;
    }

    if args.len() < 3 {
        println!("Please provide a command for the shortcut.\
        \nDo \"shrtc help\" to get a list of the default commands and their usage.");
        return;
    }

    let name = &args[1];
    let mut command = String::new();

    for word in &args[2..] {
        command.push_str(word);
        command.push(' ');
    }

    if !data::add(&Shortcut::new(name.as_str(), command.as_str(), false)) {
        println!("{} already exists", name);
    } else {
        println!("{} added successfully", name);
    }
}