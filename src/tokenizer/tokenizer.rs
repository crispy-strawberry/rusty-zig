use crate::tokenizer::{Pos, Token, TokenType};

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
        Self {
            src,
            line: 0,
            pos: 0,
        }
    }

    fn curr(&self) -> Option<u8> {
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
                b'a'..=b'z' | b'A'..=b'Z' => {
                    let pos = self.pos;
                    let mut identifier = Vec::new();
                    identifier.push(c);
                    'identifier: while let Some(character) = self.peek() {
                        if character.is_ascii_alphanumeric() {
                            identifier.push(character);
                            self.advance();
                        } else {
                            break 'identifier;
                        }
                    }
                    let pos = Pos::new(pos, self.pos, self.line);
                    Some(Token::new(
                        pos,
                        TokenType::String(String::from_utf8(identifier).unwrap()),
                    ))
                }
                b'0'..=b'9' => {
                    self.pos += 1;
                    let pos = Pos::new(self.pos, self.pos + 1, self.line);
                    Some(Token::new(pos, TokenType::Integer(12)))
                }
                b'\n' => {
                    while let Some(character) = self.peek() {
                        if character == b'\n' {
                            self.line += 1;
                            self.advance();
                        } else {
                            return self.next_token();
                        }
                    }
                    None
                }
                _ => panic!("Unidenfied character {c}"),
            }
        } else {
            None
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
