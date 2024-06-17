extern crate base;

use crossterm::style::{Color, Stylize};
use base::data;
use base::data::shortcut::Shortcut;

fn main() {
    print_list(data::all());
}

fn print_list(shortcuts: Vec<Shortcut>) {
    println!("{}", "Shortcuts:".yellow().underline(Color::DarkYellow).bold());
    for mut shortcut in shortcuts {
        let command = shortcut.command().clone();
        let name = shortcut.name().clone();
        println!("  {} -> {}", name.cyan().bold(), command);
    }
}
