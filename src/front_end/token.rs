struct Token<'a> {
    token_type: TokenType,
    value: &'a str,
    loc: Location,
}

enum TokenType {
    // Identifier
    Identifier(String),

    // Literals
    IntegerLiteral(String),
    FloatLiteral(String),
    CharLiteral(String),
    StringLiteral(String),
    MultilineStringLiteral(String),

    // Delimiters
    LeftCircleBracket,
    RightCircleBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Colon,
    DoubleColon,
    Semicolon,
    Dot,
    Comma,

    // Operators
    Equal,
    Bang,
    Plus,
    Minus,
    Asterisk,
    Ampersand,
    Slash,
    Percent,
    AmpersandEqual,
    Tilde,
    Pipe,
    PipeEqual,
    PlusEqual,
    MinusEqual,
    CaretEqual,
    Caret,
    AsteriskEqual,
    SlashEqual,
    PercentEqual,
    EqualEqual,
    BangEqual,
    RightAngleBracket,
    RightAngleBracketRightAngleBracketEqual,
    RightAngleBracketRightAngleBracket,
    RightAngleBracketEqual,
    LeftAngleBracket,
    LeftAngleBracketLeftAngleBracket,
    LeftAngleBracketLeftAngleBracketEqual,
    LeftAngleBracketEqual,
    SkinnyArrow,
    FatArrow,
    Ellipsis,
    EllipsisEqual,

    // Keywords
    And,
    As,
    Break,
    Continue,
    Else,
    Enum,
    Defer,
    False,
    For,
    Func,
    If,
    Impl,
    Import,
    In,
    Interface,
    Match,
    None,
    Or,
    Package,
    Pub,
    Return,
    Struct,
    This,
    True,
    Var,
    While,

    // Special tokens
    Eof,
    Illegal,
}

struct Location {
    line: usize,
    column: usize
}

// lazy_static! {
//     pub static ref KEYWORD_MAP: HashMap<&'static str, TokenType> = {
//         let mut m = HashMap::new();
//         m.insert("and", TokenType::And);
//         m.insert("as", TokenType::As);
//         m.insert("break", TokenType::Break);
//         m.insert("continue", TokenType::Continue);
//         m.insert("else", TokenType::Else);
//         m.insert("enum", TokenType::Enum);
//         m.insert("defer", TokenType::Defer);
//         m.insert("False", TokenType::False);
//         m.insert("func", TokenType::Func);
//         m.insert("for", TokenType::For);
//         m.insert("if", TokenType::If);
//         m.insert("impl", TokenType::Impl);
//         m.insert("import", TokenType::Import);
//         m.insert("in", TokenType::In);
//         m.insert("interface", TokenType::Interface);
//         m.insert("match", TokenType::Match);
//         m.insert("None", TokenType::None);
//         m.insert("or", TokenType::Or);
//         m.insert("package", TokenType::Package);
//         m.insert("pub", TokenType::Pub);
//         m.insert("return", TokenType::Return);
//         m.insert("struct", TokenType::Struct);
//         m.insert("this", TokenType::This);
//         m.insert("True", TokenType::True);
//         m.insert("var", TokenType::Var);
//         m.insert("while", TokenType::While);
//         m
//     };
// }
