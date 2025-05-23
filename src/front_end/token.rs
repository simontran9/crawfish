//! Token and token-related data structures

/// Token type
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// The token's type.
/// The main categories are:
/// - Identifiers
/// - Literals
/// - Delimiters
/// - Operators
/// - Keywords
/// - Special token (i.e. EOF)
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // NOTE:
    // We could have easily attached the lexemes to a variant (e.g. Identifier(String)),
    // but this cause the enum to take up quite some space, since the size of an enum is
    // the size of its largest variant.
    // Instead, we will store the lexeme as a slice for identifiers and literals.

    // Identifier
    Identifier,

    // Literals
    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    MultilineStringLiteral,

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
    Break,
    Continue,
    Const,
    Else,
    Enum,
    Defer,
    False,
    For,
    Func,
    If,
    Implements,
    Import,
    In,
    Interface,
    Match,
    Null,
    Or,
    Pub,
    Return,
    Struct,
    This,
    True,
    Var,
    While,

    // Special token
    EOF,
}

/// The location of the token within the source, bounded by an inclusive `start`, and a exclusive `end`.
#[derive(Debug, PartialEq)]
pub struct Span {
    // inclusive
    pub start: usize,
    // exclusive
    pub end: usize,
}

impl Span {
    /// Returns a span
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

impl Token {
    /// Returns a token
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }

    /// Lazily returns the lexeme as needed
    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start..self.span.end]
    }

    pub fn lexeme_token_kind(ident: &str) -> TokenKind {
        match ident {
            "and" => TokenKind::And,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "const" => TokenKind::Const,
            "else" => TokenKind::Else,
            "enum" => TokenKind::Enum,
            "defer" => TokenKind::Defer,
            "false" => TokenKind::False,
            "func" => TokenKind::Func,
            "for" => TokenKind::For,
            "if" => TokenKind::If,
            "implements" => TokenKind::Implements,
            "import" => TokenKind::Import,
            "in" => TokenKind::In,
            "interface" => TokenKind::Interface,
            "match" => TokenKind::Match,
            "null" => TokenKind::Null,
            "or" => TokenKind::Or,
            "pub" => TokenKind::Pub,
            "return" => TokenKind::Return,
            "struct" => TokenKind::Struct,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }
}
