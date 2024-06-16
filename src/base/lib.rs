pub mod data;
use data::command::Command;

pub fn test() {
    let test1 = Command::new("add", "shrtc-add", true);
    data::add(&test1);
}