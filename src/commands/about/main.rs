use std::env;
use colored::*;
use std::cmp;
use term_size::{dimensions};

fn main() {
    print_about();
}

fn print_about() {
    let version = env!("CARGO_PKG_VERSION");
    let name = "shrtc";
    let description = "shrtc is a versatile command-line tool that allows the creation of text-based shortcuts.";


    let term_width = match dimensions() {
        Some((w, _)) => w,
        _ => 80, // Default width if terminal size cannot be decided
    };


    let top_border = format!("╔{}╗", "═".repeat(term_width-2)).bold().blue();
    let bottom_border = format!("╚{}╝", "═".repeat(term_width -2)).bold().blue();
    let name_version_line = format!("║ {} ║", centered(&format!("{} v{}", name.bold(), version), term_width+name.len()-1)).bold().blue();
    let desc_centered = &description.italic().black().on_blue().to_string();

    println!("\n{}", top_border);
    println!("{}", name_version_line);
    println!("{}", bottom_border);
    println!("\n{}", desc_centered);
}

fn centered(text: &str, width: usize) -> String {
    let padding = width.saturating_sub(text.len());
    let padding_left = padding / 2;
    let padding_right = padding - padding_left;

    format!("{:padding_left$}{}{:padding_right$}", "", text, "", padding_left = padding_left, padding_right = padding_right)
}