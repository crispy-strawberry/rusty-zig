#[derive(Debug)]
pub struct Pos {
    start: usize,
    width: usize,
    line: usize,
    // lexeme: String,
}

impl Pos {
    pub fn new(start: usize, width: usize, line: usize) -> Self {
        Self { start, width, line }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Integer(usize),
    Float(f64),
    Char(char),
    String(String),
    Keyword(KeywordType),
    PrimitiveType(PrimType),

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

    Equal,       // =
    DoubleEqual, // ==
    EqualArrow,  // =>

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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Copy, Clone)]
pub enum PrimType {
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

    C_char,
    C_short,
    C_ushort,
    C_int,
    C_uint,
    C_long,
    C_ulong,
    C_longlong,
    C_ulonglong,
    C_longdouble,

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

#[derive(Debug)]
pub struct Token(Pos, TokenType);

impl Token {
    pub fn new(pos: Pos, r#type: TokenType) -> Self {
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
}
