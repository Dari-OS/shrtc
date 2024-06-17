extern crate base;

use std::env;
use crossterm::style::{Color, Stylize};
use base::data;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a name to get.\
        \nDo \"shrtc help\" to get a list of the default commands and their usage.");
        return;
    }

    let shortcut = data::get(&args[1]);

    if let Some(mut shrtc) = shortcut {
        println!("{}", shrtc.name().as_str().yellow().underline(Color::DarkYellow).bold());
        println!("  {} ->   {}", "Command".cyan(), shrtc.command().as_str().white().italic());
        println!("  {} ->   {}", "Default".cyan(), if *shrtc.default() {"true".green()} else {"false".red()});

    }  else {
        print!("{}", format!("No shortcut was found with the name {}", &args[1]).red().bold())
    }
}