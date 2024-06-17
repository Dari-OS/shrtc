extern crate base;
use std::env;
use base::data::shortcut::Shortcut;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let about = base::data::get("about"); //If the user provides no command args, the "about" command gets executed
        match about {
            None => {
                println!("Please provide more command arguments");
            }
            Some(shortcut) => {
                execute(shortcut, "");
            }
        }
        return;
    }

    if let Some(shortcut) = base::data::get(args[1].as_str()) {
        let mut rest_args = String::from("");
        if args.len() >= 2 {
            for current_arg in &args[2..] {
                rest_args.push_str(current_arg);
                rest_args.push(' ');
            }
        }

        execute(shortcut, &rest_args);
    } else {
        println!("{} was not recognized as shortcut-name.\
                    \nDo \"shrtc help\" to get a list of the default commands and their usage.", args[1])
    }
}

fn execute(shortcut: Shortcut, arg: &str) {
    match shortcut.execute(arg) {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.is_empty() {
                println!("{}",stdout );
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                println!("{}",stderr );
            }
        }
        Err(err) => {println!("{}", err)}
    };
}