#![allow(non_snake_case)]
#![allow(unused)]

use gm::*;

fn main(){
    let input = Input::get();
    let mut tokens = lexer::create_tokens(&input);
    println!("Tokens: {:?}", tokens);
    matcher::identify_pattern(tokens, input);
}
