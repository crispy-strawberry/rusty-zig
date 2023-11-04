use rusty_zig::tokenizer::{Diagnostic, Token, TokenType};

fn main() {
    let diag = Diagnostic::new(1, 1, 1);
    let a = Token::new(diag, TokenType::Float(12.5));
    println!("{a:?}");
    println!("Hello, world!");
}
