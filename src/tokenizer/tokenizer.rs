use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::tokenizer::{KeywordType, Pos, Token, TokenType};

// lazy_static! {
pub static KEYWORD_MAP: Lazy<HashMap<&'static str, KeywordType>> = Lazy::new(|| {
    HashMap::from([
        ("addrspace", KeywordType::AddrSpace),
        ("align", KeywordType::Align),
        ("allowzero", KeywordType::AllowZero),
        ("and", KeywordType::And),
        ("anyframe", KeywordType::AnyFrame),
        ("anytype", KeywordType::AnyType),
        ("asm", KeywordType::Asm),
        ("async", KeywordType::Async),
        ("await", KeywordType::Await),
        ("break", KeywordType::Break),
        ("callconv", KeywordType::CallConv),
        ("catch", KeywordType::Catch),
        ("comptime", KeywordType::Comptime),
        ("const", KeywordType::Const),
        ("continue", KeywordType::Continue),
        ("defer", KeywordType::Defer),
        ("else", KeywordType::Else),
        ("enum", KeywordType::Enum),
        ("errdefer", KeywordType::Errdefer),
        ("error", KeywordType::Error),
        ("export", KeywordType::Export),
        ("extern", KeywordType::Extern),
        ("fn", KeywordType::Fn),
        ("for", KeywordType::For),
        ("if", KeywordType::If),
        ("inline", KeywordType::Inline),
        ("linksection", KeywordType::LinkSection),
        ("noalias", KeywordType::NoAlias),
        ("nosuspend", KeywordType::NoSuspend),
        ("opaque", KeywordType::Opaque),
        ("or", KeywordType::Or),
        ("orelse", KeywordType::OrElse),
        ("packed", KeywordType::Packed),
        ("pub", KeywordType::Pub),
        ("resume", KeywordType::Resume),
        ("return", KeywordType::Return),
        ("struct", KeywordType::Struct),
        ("suspend", KeywordType::Suspend),
        ("switch", KeywordType::Switch),
        ("test", KeywordType::Test),
        ("threadlocal", KeywordType::ThreadLocal),
        ("try", KeywordType::Try),
        ("union", KeywordType::Union),
        ("unreachable", KeywordType::Unreachable),
        ("usingnamespace", KeywordType::UsingNamespace),
        ("var", KeywordType::Var),
        ("volatile", KeywordType::Volatile),
        ("while", KeywordType::While),
    ])
});

static OPERATOR_MAP: Lazy<HashMap<&str, TokenType>> =
    Lazy::new(|| HashMap::from([("+", TokenType::Plus)]));

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
        let byte = self.curr();
        self.pos += 1;
        byte
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if let Some(c) = self.advance() {
            match c {
                // Do i really have to do this manually
                b'&' => match self.peek() {
                    Some(b'=') => {
                        self.advance();
                        let pos = Pos::new(self.pos - 1, 2, self.line);
                        Some(Token::new(pos, TokenType::AmpersandEqual))
                    }
                    _ => Some(Token::new(
                        Pos::new(self.pos, 1, self.line),
                        TokenType::Ampersand,
                    )),
                },

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
