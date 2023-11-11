use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::tokenizer::{KeywordType, Span, Token, TokenType};

use super::PrimitiveType;

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

static PRIMITIVE_TYPES: Lazy<HashMap<&'static str, PrimitiveType>> = Lazy::new(|| {
    use PrimitiveType::*;
    HashMap::from([
        ("i8", I8),
        ("u8", U8),
        ("i16", I16),
        ("u16", U16),
        ("i32", I32),
        ("u32", U32),
        ("i64", I64),
        ("u64", U64),
        ("i128", I128),
        ("u128", U128),
        ("isize", ISize),
        ("usize", USize),
        ("c_char", Char),
        ("c_short", Short),
        ("c_ushort", Ushort),
        ("c_int", Int),
        ("c_uint", Uint),
        ("c_long", Long),
        ("c_ulong", Ulong),
        ("c_longlong", Longlong),
        ("c_ulonglong", ULonglong),
        ("c_longdouble", Longdouble),
        ("f16", F16),
        ("f32", F32),
        ("f64", F64),
        ("f128", F128),
        ("bool", Bool),
        ("anyopaque", AnyOpaque),
        ("void", Void),
        ("noreturn", NoReturn),
        ("type", Type),
        ("anyerror", AnyError),
        ("comptime_int", ComptimeInt),
        ("comptime_float", ComptimeFloat),
    ])
});

static OPERATOR_MAP: Lazy<HashMap<&str, TokenType>> =
    Lazy::new(|| HashMap::from([("+", TokenType::Plus)]));

/// A helper enum to avoid repition and make the source
/// code more readable. As zig has quite a wide range of
/// operators, it is reptitative and ugly to manually check
/// them each time.
enum OperatorType {
    Other(u8),
    Same,
    Equals,
    Percent,
    PercentEquals,
    Pipe,
    PipeEquals,
}

/// So my idea for the tokenizer is that it should be
/// implement the standard iterator trait.
/// That way, we can also make it peekable easily.
/// Also, I think it's better to just use standard traits
/// rather than using something custom.
/// Ideally, I would want to use a separate `TokenStream`
/// type that can be constructed using something like
/// `Tokenizer.tokenize()`.
pub struct Tokenizer {
    src: String,
    pos: usize,
    col: usize,
    line: usize,
}

impl Tokenizer {
    pub fn new(src: String) -> Self {
        Self {
            src,
            pos: 0,
            col: 0,
            line: 0,
        }
    }

    /// Returns the current byte in the src.
    /// According to me atleast, the operation shouldn't
    /// be expensive as it just consists of very cheap functions
    fn current(&self) -> Option<u8> {
        self.src.as_bytes().get(self.pos).copied()
    }

    fn peek(&self) -> Option<u8> {
        self.src.as_bytes().get(self.pos + 1).copied()
    }

    /// Takes the cursor forward 1 unit, emitting the token consumed
    fn advance(&mut self) -> Option<u8> {
        let byte = self.current();
        self.pos += 1;
        self.col += 1;
        byte
    }

    fn advance_steps(&mut self, step: usize) {
        self.pos += step;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if let Some(c) = self.advance() {
            match c {
                b'{' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::LBrace))
                }
                b'[' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::LBracket))
                }
                b'(' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::LParen))
                }
                b'}' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::RBrace))
                }
                b']' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::RBracket))
                }
                b')' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::RParen))
                }
                b';' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::Semicolon))
                }
                b':' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::Colon))
                }
                b',' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::Comma))
                }
                b'~' => {
                    let span = Span::new(self.col - 1, 1, self.line);
                    Some(Token(span, TokenType::Tilde))
                }
                // Do i really have to do this manually
                b'&' => match self.current() {
                    Some(b'=') => {
                        self.advance();
                        let pos = Span::new(self.col - 2, 2, self.line);
                        Some(Token(pos, TokenType::AmpersandEqual))
                    }
                    _ => Some(Token(
                        Span::new(self.col - 1, 1, self.line),
                        TokenType::Ampersand,
                    )),
                },
                b'*' => match self.current() {
                    Some(b'*') => {
                        self.advance();
                        let span = Span::new(self.col - 1, 2, self.line);
                        Some(Token(span, TokenType::Asterisk2))
                    }
                    Some(b'=') => {
                        self.advance();
                        let span = Span::new(self.col - 1, 2, self.line);
                        Some(Token(span, TokenType::AsteriskEqual))
                    }
                    _ => {
                        let span = Span::new(self.col - 1, 1, self.line);
                        Some(Token(span, TokenType::Asterisk))
                    }
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
                    let span = Span::new(pos, self.pos, self.line);
                    Some(Token(
                        span,
                        TokenType::String(String::from_utf8(identifier).unwrap()),
                    ))
                }
                b'0'..=b'9' => {
                    self.pos += 1;
                    let span = Span::new(self.pos, self.pos + 1, self.line);
                    Some(Token(span, TokenType::Integer(12)))
                }
                b' ' | b'\t' | b'\n' | b'\r' => {
                    if c == b'\n' {
                        self.line += 1;
                        self.col = 0;
                    }
                    while let Some(w) = self.current() {
                        match w {
                            b' ' | b'\t' | b'\r' => {
                                self.advance();
                            }
                            b'\n' => {
                                self.pos += 1;
                                self.line += 1;
                                self.col = 0;
                            }
                            _ => return self.next_token(),
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
