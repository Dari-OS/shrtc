use crossterm::style::{Color, Stylize};


fn main() {
    print_help();
}

fn print_help() {
    println!("{}", "Commands:".yellow().underline(Color::DarkYellow));
    println!("  {} - {} {}", "about".cyan().bold(), "Shows your some information about", "shrtc".bold());
    println!("  {} - {}", "help".cyan().bold(), "Shows all standard shortcuts and their usage");
    println!("  {} - {}", "github".cyan().bold(), "Shows my GitHub profile");
    println!("  {} {} - {}", "add".cyan().bold(), "<name> <command>".cyan().italic(), "Adds a new shortcut to the list");
    println!("  {} {} - {}", "rm".cyan().bold(), "<name>".cyan().italic(), "Removes a specific shortcut");
    println!("  {} {} - {}", "get".cyan().bold(), "<name>".cyan().italic(), "Gets information about a specific shortcut");
    println!("  {} - {}", "list".cyan().bold(), "Lists all shortcuts and their associated names");

}
