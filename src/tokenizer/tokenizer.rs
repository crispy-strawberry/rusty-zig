use crate::tokenizer::{ Token, TokenType, Diagnostic };

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

    pub fn next(&mut self) -> Token {
        match self.curr() {
            Some(c) => { 
                self.pos += 10;
                Token::new(Diagnostic::new(self.pos, 10, self.line), TokenType::Float(4.5))
            }
            None => Token::new(Diagnostic::new(self.pos+1, 0, self.line), TokenType::EOF)
        }
    }
}
