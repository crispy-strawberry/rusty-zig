use rusty_zig::tokenizer::{Span, Token, Tokenizer};

fn main() {
    let src = String::from(
        "{[% }](  %= ;
[; , &, . .? .* °⁂😭
        :༼ つ ◕_◕ ༽つ
        == , => != ! ...
        ^ ^= |&
        ,)&={}&(||)",
    );
    
    let tokenizer = Tokenizer::new(src);
    for tok in tokenizer {
        println!("{tok:?}");
    }
}
