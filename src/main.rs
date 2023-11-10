use rusty_zig::tokenizer::{Span, Token, TokenType, Tokenizer};

fn main() {
    let src = String::from(
        "{[ }](   ;

        ,)&={}&()",
    );
    let tokenizer = Tokenizer::new(src);
    for i in tokenizer {
        println!("{i:?}");
    }
    println!("Hello, world!");
}
