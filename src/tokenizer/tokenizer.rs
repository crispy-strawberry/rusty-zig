use crate::tokenizer::{ Token, TokenType, Pos };

/// So my idea for the tokenizer is that it should be
/// implement the standard iterator trait.
/// That way, we can also make it peekable easily.
/// Also, I think it's better to just use standard traits
/// rather than using something custom.
/// Ideally, I would want to use a separate `TokenStream`
/// type that can be constructed using something like
/// `Tokenizer.tokenize()`.
struct Tokenizer {
    src: String,
    pos: usize,
    line: usize,
}


impl Tokenizer {
    pub fn new(src: String) -> Self {
        Self { src, line: 0, pos: 0 }
    }

    fn curr(&self) -> Option<u8> {
        // self.src.as_bytes().to_owned()
        self.src.as_bytes().get(self.pos).copied()
    }

    fn peek(&self) -> Option<u8> {
        self.src.as_bytes().get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let byte = self.src.as_bytes().get(self.pos).copied();
        self.pos += 1;
        byte
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if let Some(c) = self.advance() {
            match c {
                b'a'..=b'z' => {
                    self.pos += 1;
                    let diag = Pos::new(self.pos, self.pos+1, self.line);
                    Some(Token::new(diag, TokenType::String(String::from(char::from_u32(c.into()).unwrap()))))
                }
                 _ => unimplemented!(),
            }
        }
        None
    }
}


impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
