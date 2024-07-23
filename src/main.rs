use gm::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = Input::get();
    let tokens = lexer::create_tokens(&input);
    println!("Tokens: {:?}", tokens);
    matcher::identify_pattern(tokens, input).await?;
    Ok(())
}
