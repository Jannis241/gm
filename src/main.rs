#![allow(non_snake_case)]
#![allow(unused)]

use gm::*;

mod terminal;
mod matcher;
mod lexer;
mod git_commands;
mod config_manager;
mod helper;

fn main(){
    let input = Input::get();
    let mut tokens = lexer::create_tokens(&input);
    matcher::identify_pattern(tokens, input);
    
}
