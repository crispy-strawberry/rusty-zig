use rusty_zig::tokenizer::{Pos, Token, TokenType};

fn main() {
    let diag = Pos::new(1, 1, 1);
    let a = Token::new(diag, TokenType::Float(12.5));
    println!("{a:?}");
    println!("Hello, world!");
}
