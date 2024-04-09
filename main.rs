use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("enter payload here");
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    let output = Command::new("./test.bat")
                         .arg(input.trim())
                         .output()
                         .expect("Failed to execute command");
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
}
