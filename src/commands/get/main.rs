use std::env;

fn main() {
    println!("Get: {}", env::args().collect())
}