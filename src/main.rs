use rusty_zig::tokenizer::{Span, Token, Tokenizer};

fn main() {
    let src = String::from(
        "{[% }](  %= ;
[; , &, . .? .* ©
        == , => != ! ...
        ^ ^= |&
        ,)&={}&(||)",
    );
    let tokenizer = Tokenizer::new(src);
    let tokens: Vec<Token> = tokenizer.collect();
    println!("{:?}", tokens);
}
