use rusty_zig::tokenizer::{Span, Token, TokenType};

fn main() {
    let span = Span::new(1, 1, 1);
    let a = Token::new(span, TokenType::Float(12.5));
    println!("{a:?}");
    println!("Hello, world!");
}
