#[derive(Debug)]
pub struct Diagnostic {
    start: usize,
    end: usize,
    line: usize,
    // lexeme: String,
}

impl Diagnostic {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Self { start, end, line }
    }
}

#[derive(Debug)]
pub enum TokenType {
    Integer(usize),
    String(String),
    Float(f64),
    Keyword(KeywordType),
}

#[derive(Debug)]
pub enum KeywordType {
    While,
    For,
    Var,
    Const,
    Fn,
    Enum,
    Union,
}

#[derive(Debug)]
pub struct Token(Diagnostic, TokenType);

impl Token {
    pub fn new(diag: Diagnostic, r#type: TokenType) -> Self {
        Self(diag, r#type)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self.1, TokenType::Keyword(_))
    }

    pub fn is_int(&self) -> bool {
        matches!(self.1, TokenType::Integer(_))
    }
}
