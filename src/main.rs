use rusty_zig::tokenizer::Tokenizer;

fn main() {
    let src = String::from(
        "{[ }](   ;
[; , &,]
        ,)&={}&( )",
    );
    let tokenizer = Tokenizer::new(src);
    for tok in tokenizer {
        println!("{tok:?}");
    }
}
