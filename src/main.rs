use rusty_zig::tokenizer::{ Token, TokenType, Diagnostic };

fn main() {
    let diag = Diagnostic::new(1, 1, 1);
    let a = Token::new(diag, TokenType::Float(12.5));
    println!("{a:?}");
    println!("Hello, world!");
}
