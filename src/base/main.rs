extern crate base;

use std::env;
use colored::*;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_about();
        return;
    }
}


fn print_about() {
    let version = env!("CARGO_PKG_VERSION");
    let name = "shrtc";
    let description = "shrtc is a shortcut CLI application that allows the user to associate a command with a name.";
    let list_command = "You can list all associated names with: shrtc list";
    let help_command = "Get help with: shrtc help";

    // Calculate the maximum width required for centering
    let max_width = cmp::max(name.len() + version.len() + 3, description.len());

    // Helper function to center text within a given width
    fn centered(text: &str, width: usize) -> String {
        let padding = width.saturating_sub(text.len()) / 2;
        format!("{:padding$}{}{:padding$}", "", text, "")
    }

    let top_border = format!(
        "╔═{}═╗",
        "═".repeat(max_width + 2)
    ).bold().blue();
    let bottom_border = format!(
        "╚═{}═╝",
        "═".repeat(max_width + 2)
    ).bold().blue();
    let name_version_line = format!(
        "║ {} ║",
        centered(&format!("{} v{}", name.bold(), version), max_width)
    ).bold().blue();

    println!("\n{}", top_border);
    println!("{}", name_version_line);
    println!("{}", bottom_border);
    println!("\n{}", centered(description.italic().white().on_bright_blue().bold().to_string().as_str(), max_width + 4));
    println!();
    println!("{}", "Commands:".bold().underline().yellow());
    println!("  {} - {}", "list".cyan(), list_command);
    println!("  {} - {}", "help".cyan(), help_command);
}