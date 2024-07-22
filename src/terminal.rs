#![allow(non_snake_case)]
#![allow(unused)]
use crate::*;

pub fn throw_error(msg: &str){
    println!("{}: {}", "gm", msg)
}

pub fn new_lines(num_of_lines: usize) {
    println!("{}", "\n".repeat(num_of_lines));
}


pub fn print_command_help(){
    println!("user wants overall command help")
}


pub fn clear_terminal() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}
pub fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().to_string()
}


