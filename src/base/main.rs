extern crate base;
use std::{env, thread};
use std::io::Read;
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
    let mut child = match shortcut.execute(arg) {
        Ok(child) => child,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    let stdout_thread = thread::spawn(move || {
        let mut buf = [0; 1024];
        while let Ok(bytes_read) = stdout.read(&mut buf) {
            if bytes_read == 0 {
                break;
            }
            print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
        }
    });

    let stderr_thread = thread::spawn(move || {
        let mut buf = [0; 1024];
        while let Ok(bytes_read) = stderr.read(&mut buf) {
            if bytes_read == 0 {
                break;
            }
            eprint!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
        }
    });

    let _ = child.wait().unwrap(); // Wait for the process to finish
    let _ = stdout_thread.join().unwrap(); // Wait for stdout thread
    let _ = stderr_thread.join().unwrap(); // Wait for stderr thread
    println!()
}