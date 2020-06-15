use std::io;

pub fn read_line() -> String {
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}