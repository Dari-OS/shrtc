extern crate base;
use base::data;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a name to remove.\
        \nDo \"shrtc help\" to get a list of the default commands and their usage.");
        return;
    }

    let name = &args[1];

    let status_code = data::remove_safely(name);
    if  status_code == 0{
        println!("{} was removed successfully", name);
    } else if status_code == 1 {
        println!("{} does not exist", name );
    } else {
        println!("{} cannot be removed because it is a default command", name );
    }
}