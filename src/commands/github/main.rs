use std::process::Command;

fn open_url(url: &str) {
    let default_message = format!("This is the link to my GitHub page: {}
                                        \nThanks for visiting my GitHub profile!", url);

    match std::env::consts::OS {
        "windows" => {
            if let Err(_) = Command::new("cmd")
                .args(&["/C", "start", "", url])
                .spawn() {
                println!("{}", &default_message);
            }
        },
        "macos" | "ios" => {
            if let Err(_) = Command::new("open")
                .arg(url)
                .spawn() {
                println!("{}", &default_message);
            }
        },
        "linux" | "freebsd" | "dragonfly" | "netbsd" | "openbsd" | "solaris" => {
            if let Err(_) = Command::new("xdg-open")
                .arg(url)
                .spawn() {
                println!("{}", &default_message);
            }
        },
        "android" => {
            if let Err(_) = Command::new("am")
                .arg("start")
                .arg("-a")
                .arg("android.intent.action.VIEW")
                .arg("-d")
                .arg(url)
                .spawn() {
                println!("{}", &default_message);
            }
        },
        _ => {
            println!("{}", &default_message);
        }
    };


}

fn main() {
    let url = "https://github.com/Dari-OS/";
    open_url(url);
}
