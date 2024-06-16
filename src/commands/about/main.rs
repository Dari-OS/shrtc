use std::env;
use crossterm::{terminal};
use crossterm::style::{ResetColor, Stylize};


fn main() {
    print_about();
}

fn print_about() {
    let version = env!("CARGO_PKG_VERSION");
    let name = "shrtc";
    let description = "shrtc is a versatile command-line tool that allows the creation of text-based shortcuts.";

    // Get terminal dimensions
    let (term_width, _) = terminal::size().unwrap_or((80, 0));

    // Prepare borders and text
    let top_border = format!("{}", "═".repeat((term_width - 2) as usize));
    let bottom_border = format!("{}", "═".repeat((term_width - 2) as usize));

    let name_version_line = format!("{}", centered(&format!("{} {}", name.bold(), version), term_width as usize + name.len() +1));

    let desc_centered = centered(&description.italic().black().on_blue().to_string(), term_width as usize + (name.len()  + version.len() +3) * 2);

    let line  = "║".bold().blue();
    println!("{}", format!("╔{}╗", top_border).bold().blue());
    println!("{}", format!("{}{}{}",&line, name_version_line.bold().blue(), &line));
    println!("{}", format!("╚{}╝", bottom_border).bold().blue());
    println!();
    println!("{}", format!("{}\n", desc_centered));
    println!("{}", ResetColor);
}

fn centered(text: &str, width: usize) -> String {
    let padding = width.saturating_sub(text.len());
    let padding_left = padding / 2;
    let padding_right = padding - padding_left;

    format!("{:padding_left$}{}{:padding_right$}", "", text, "", padding_left = padding_left, padding_right = padding_right)
}
