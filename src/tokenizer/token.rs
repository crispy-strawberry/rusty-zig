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
    EOF,
}

#[derive(Debug)]
pub enum KeywordType {
    AddrSpace,
    Align,
    AllowZero,
    And,
    AnyFrame,
    AnyType,
    Asm,
    Async,
    Await,
    Break,
    CallConv,
    Catch,
    Comptime,
    Const,
    Continue,
    Defer,
    Else,
    Enum,
    Errdefer,
    Error,
    Export,
    Extern,
    Fn,
    For,
    If,
    Inline,
    LinkSection,
    NoAlias,
    NoSuspend,
    Opaque,
    Or,
    OrElse,
    Packed,
    Pub,
    Resume,
    Return,
    Struct,
    Suspend,
    Switch,
    Test,
    ThreadLocal,
    Try,
    Union,
    Unreachable,
    UsingNamespace,
    Var,
    Volatile,
    While,
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
