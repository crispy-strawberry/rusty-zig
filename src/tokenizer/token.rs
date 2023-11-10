/// It is used to specify the location of
/// tokens in the source text. As far as I know,
/// Zig doesn't have support for multi line tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub(crate) col: usize,
    pub(crate) width: usize,
    pub(crate) line: usize,
    // lexeme: String,
}

impl Span {
    pub fn new(col: usize, width: usize, line: usize) -> Self {
        Self { col, width, line }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// A comptime_int
    Integer(usize),

    /// A comptime_float
    Float(f64),
    Char(char),
    String(String),
    Keyword(KeywordType),
    PrimitiveType(PrimitiveType),

    Ampersand,      // &
    AmpersandEqual, // &=

    Asterisk,             // *
    Asterisk2,            // **
    AsteriskEqual,        // *=
    AsteriskPercent,      // *%
    AsteriskPercentEqual, // *%=
    AsteriskPipe,         // *|
    AsteriskPipeEqual,    // *|=

    Caret,      // ^
    CaretEqual, // ^=

    Colon, // :
    Comma, // ,

    Dot,             // .
    Dot2,            // ..
    Dot3,            // ...
    DotAsterisk,     // .*
    DotQuestionMark, // .?

    Equal,      // =
    EqualEqual, // ==
    EqualArrow, // =>

    ExclamationMark,      // !
    ExclamationMarkEqual, // !=

    LArrow,           // <
    LArrow2,          // <<
    LArrow2Equal,     // <<=
    LArrow2Pipe,      // <<|
    LArrow2PipeEqual, // <<|=
    LArrowEqual,      // <=

    LBrace,   // {
    LBracket, // [
    LParen,   // (

    Minus,             // -
    MinusEqual,        // -=
    MinusPercent,      // -%
    MinusPercentEqual, // -%=,
    MinusPipe,         // -|
    MinusPipeEqual,    // -|=
    MinusArrow,        // ->

    Percent,      // %
    PercentEqual, // %=

    Pipe,      // |
    Pipe2,     // ||
    PipeEqual, // |=

    Plus,             // +
    Plus2,            // ++
    PlusEqual,        // +=
    PlusPercent,      // +%
    PlusPercentEqual, // +%=
    PlusPipe,         // +|
    PlusPipeEqual,    // +|=

    QuestionMark, // ?

    RArrow,       // >
    RArrow2,      // >>
    RArrow2Equal, // >>=
    RArrowEqual,  // >=

    RBrace,   // }
    RBracket, // ]
    RParen,   // )

    Semicolon, // ;

    Slash,      // /
    SlashEqual, // /=

    Tilde, // ~
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PrimitiveType {
    I8,
    U8,

    I16,
    U16,

    I32,
    U32,

    I64,
    U64,

    I128,
    U128,

    ISize,
    USize,

    Char,
    Short,
    Ushort,
    Int,
    Uint,
    Long,
    Ulong,
    Longlong,
    ULonglong,
    Longdouble,

    F16,
    F32,
    F64,
    F80,
    F128,

    Bool,
    AnyOpaque,
    Void,
    NoReturn,
    Type,
    AnyError,
    ComptimeInt,
    ComptimeFloat,
}

#[derive(Debug, Clone)]
pub struct Token(pub(crate) Span, pub(crate) TokenType);

impl Token {
    pub fn new(pos: Span, r#type: TokenType) -> Self {
        Self(pos, r#type)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self.1, TokenType::Keyword(_))
    }

    pub fn is_int(&self) -> bool {
        matches!(self.1, TokenType::Integer(_))
    }

    pub fn is_token_type(&self, token_type: TokenType) -> bool {
        matches!(&self.1, token_type)
    }

    /// Checks if the token is an assign op.
    /// This will be useful when parsing.
    pub fn is_assign_op(&self) -> bool {
        match &self.1 {
            TokenType::AsteriskEqual
            | TokenType::AsteriskPipeEqual
            | TokenType::SlashEqual
            | TokenType::PercentEqual
            | TokenType::PlusEqual
            | TokenType::PlusPipeEqual
            | TokenType::MinusEqual
            | TokenType::MinusPipeEqual
            | TokenType::LArrow2Equal
            | TokenType::LArrow2PipeEqual
            | TokenType::RArrow2
            | TokenType::AmpersandEqual
            | TokenType::CaretEqual
            | TokenType::PipeEqual
            | TokenType::AsteriskPercentEqual
            | TokenType::PlusPercentEqual
            | TokenType::MinusPercentEqual
            | TokenType::Equal => true,

            _ => false,
        }
    }

    pub fn is_compare_op(&self) -> bool {
        match &self.1 {
            TokenType::EqualEqual
            | TokenType::ExclamationMarkEqual
            | TokenType::LArrow
            | TokenType::RArrow
            | TokenType::LArrowEqual
            | TokenType::RArrowEqual => true,

            _ => false,
        }
    }

    pub fn is_bitwise_op(&self) -> bool {
        match &self.1 {
            TokenType::Ampersand
            | TokenType::Caret
            | TokenType::Pipe
            | TokenType::Keyword(KeywordType::OrElse)
            | TokenType::Keyword(KeywordType::Catch) => true,

            _ => false,
        }
    }
}
