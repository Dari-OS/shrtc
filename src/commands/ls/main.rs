use std::env;

fn main() {
    println!("ls: {}", env::args().collect())
}