use std::{iter::Peekable, str::CharIndices};

use crate::front_end::token::{Token, TokenKind, Span};

enum TokenizerError {
    InvalidCharacter,
}

/// Tokenizer
pub struct Tokenizer<'a> {
    // Original source code
    source: &'a str,
    // Iterable source code
    chars: Peekable<CharIndices<'a>>,
}

// TODO: handle const BOM: &str = "\u{FEFF}";
// TODO: implement regular string
// TODO: implement multi string
// TODO: implement char
impl<'a> Tokenizer<'a> {
    /// Returns a tokenizer iterator
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
        }
    }

    /// Returns the next token
    pub fn next(&mut self) -> Result<Token, TokenizerError> {
        let (token_kind, start) = self.determine_token_kind();
        let end = 2;
        Ok(Token::new(token_kind, Span::new(start, end)))
    }

    fn determine_token_kind(&mut self) -> Result<(TokenKind, usize), TokenizerError> {
        while let Some((start, c)) = self.chars.next() {
            match c {
                c if c.is_whitespace() => continue,
                '(' => return Ok((TokenKind::LeftCircleBracket, start)),
                ')' => return Ok((TokenKind::RightCircleBracket, start)),
                '{' => return Ok((TokenKind::LeftCurlyBracket, start)),
                '[' => return Ok((TokenKind::LeftSquareBracket, start)),
                ']' => return Ok((TokenKind::RightSquareBracket, start)),
                // _ => return error then match in `next()`
            }
        }
        Ok((TokenKind::EOF, self.source.len()))
    }
}

// current_index: usize,
// read_index: usize,
// current_char: Option<char>,

//         '&' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.AmpersandEqual, null);
//             } else {
//                 token = Token.init(Token.TokenType.Ampersand, null);
//             }
//         },
//         '~' => token = Token.init(Token.TokenType.Tilde, null),
//         '|' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.PipeEqual, null);
//             } else {
//                 token = Token.init(Token.TokenType.Pipe, null);
//             }
//         },
//         '^' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.CaretEqual, null);
//             } else {
//                 token = Token.init(Token.TokenType.Caret, null);
//             }
//         },
//         ':' => token = Token.init(Token.TokenType.Colon, null),
//         ';' => token = Token.init(Token.TokenType.Semicolon, null),
//         '.' => {
//             if (self.peek() == '.') {
//                 self.read();
//                 if (self.peek() == '=') {
//                     self.read();
//                     token = Token.init(Token.TokenType.EllipsisEqual, null);
//                 } else {
//                     token = Token.init(Token.TokenType.Ellipsis, null);
//                 }
//             } else {
//                 token = Token.init(Token.TokenType.Dot, null);
//             }
//         },
//         ',' => token = Token.init(Token.TokenType.Comma, null),
//         '=' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.EqualEqual, null);
//             } else if (self.peek() == '>') {
//                 self.read();
//                 token = Token.init(Token.TokenType.FatArrow, null);
//             } else {
//                 token = Token.init(Token.TokenType.Equal, null);
//             }
//         },
//         '!' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.BangEqual, null);
//             } else {
//                 token = Token.init(Token.TokenType.Bang, null);
//             }
//         },
//         '+' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.PlusEqual, null);
//             } else {
//                 token = Token.init(Token.TokenType.Plus, null);
//             }
//         },
//         '-' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.MinusEqual, null);
//             } else if (self.peek() == '>') {
//                 self.read();
//                 token = Token.init(Token.TokenType.SkinnyArrow, null);
//             } else {
//                 token = Token.init(Token.TokenType.Minus, null);
//             }
//         },
//         '*' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.AsteriskEqual, null);
//             } else {
//                 token = Token.init(Token.TokenType.Asterisk, null);
//             }
//         },
//         '/' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.SlashEqual, null);
//             } else if (self.peek() == '/') {
//                 self.skip_line_comment();
//                 return self.next();
//             } else {
//                 token = Token.init(Token.TokenType.Slash, null);
//             }
//         },
//         '%' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.PercentEqual, null);
//             } else {
//                 token = Token.init(Token.TokenType.Percent, null);
//             }
//         },
//         '>' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.RightAngleBracketEqual, null);
//             } else if (self.peek() == '>') {
//                 self.read();
//                 if (self.peek() == '=') {
//                     self.read();
//                     token = Token.init(Token.TokenType.RightAngleBracketRightAngleBracketEqual, null);
//                 } else {
//                     token = Token.init(Token.TokenType.RightAngleBracketRightAngleBracket, null);
//                 }
//             } else {
//                 token = Token.init(Token.TokenType.RightAngleBracket, null);
//             }
//         },
//         '<' => {
//             if (self.peek() == '=') {
//                 self.read();
//                 token = Token.init(Token.TokenType.LeftAngleBracketEqual, null);
//             } else if (self.peek() == '<') {
//                 self.read();
//                 if (self.peek() == '=') {
//                     self.read();
//                     token = Token.init(Token.TokenType.LeftAngleBracketLeftAngleBracketEqual, null);
//                 } else {
//                     token = Token.init(Token.TokenType.LeftAngleBracketLeftAngleBracket, null);
//                 }
//             } else {
//                 token = Token.init(Token.TokenType.LeftAngleBracket, null);
//             }
//         },
//         'a'...'z', 'A'...'Z', '_' => {
//             const lexeme = self.read_identifier();
//             if (Token.keyword_map.get(lexeme)) |token_type_keyword| {
//                 token = Token.init(token_type_keyword, null);
//             } else {
//                 token = Token.init(Token.TokenType.Identifier, lexeme);
//             }
//         },
//         '0'...'9' => {
//             if (self.peek() == '.') {
//                 const lexeme = self.read_float();
//                 if (lexeme == null) {
//                     token = Token.init(Token.TokenType.Illegal, null);
//                 } else {
//                     token = Token.init(Token.TokenType.FloatLiteral, lexeme);
//                 }
//             } else {
//                 token = Token.init(Token.TokenType.IntegerLiteral, self.read_integer());
//             }
//         },
//         ascii_null => token = Token.init(Token.TokenType.Eof, null),
//         else => token = Token.init(Token.TokenType.Illegal, null),
//     }
//     self.read();
//     return token;
// }

// skips line comment
// fn skip_line_comment(self: *Scanner) void {
//     while (self.current_char != '\n') {
//         self.read();
//     }
// }

// checks the character ahead to decide on a token
// fn peek(self: *Scanner) u8 {
//     if (self.read_index >= self.source.len) {
//         return ascii_null;
//     }
//     return self.source[self.read_index];
// }

// reads the next character and sets current_char to it
// fn read(self: *Scanner) void {
//     if (self.read_index >= self.source.len) {
//         self.current_char = ascii_null;
//     } else {
//         self.current_char = self.source[self.read_index];
//     }
//     self.current_index = self.read_index;
//     self.read_index += 1;
// }

// reads the supposed identifier
// fn read_identifier(self: *Scanner) []const u8 {
//     const initial_index = self.current_index;
//     while (std.ascii.isAlphabetic(self.peek()) or std.ascii.isDigit(self.peek()) or self.peek() == '_') {
//         self.read();
//     }
//     return self.source[initial_index .. self.current_index + 1];
// }

// fn lex_identifier(&mut Self) -> Token {
//     let mut ident = String::new();

//     while let Some(&c) = self.peek() {
//         if c.isalphanumeric() || c == '_' {
//             ident.push(self.read().unwrap());
//         } else {
//             break;
//         }
//     }

//     Token::Identifier(ident)
// }

// reads the supposed float
// fn read_float(self: *Scanner) ?[]const u8 {
//     const initial_index = self.current_index;

//     // in a supposed float "num.c1c2c3...", the following
//     // two lines reads '.' and 'c1' then stops
//     self.read();
//     self.read();

//     // if c1 isn't a digit, it's definitely not a float
//     if (!std.ascii.isDigit(self.current_char)) {
//         return null;
//     }

//     while (std.ascii.isDigit(self.peek())) {
//         self.read();
//     }
//     return self.source[initial_index .. self.current_index + 1];
// }

// reads an integer
// fn read_integer(self: *Scanner) []const u8 {
//     const initial_index = self.current_index;

//     while (std.ascii.isDigit(self.peek())) {
//         self.read();
//     }
//     return self.source[initial_index .. self.current_index + 1];
// }
// }

fn match_keyword(ident: &str) -> TokenKind {
    match ident {
        "and" => TokenKind::And,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "else" => TokenKind::Else,
        "enum" => TokenKind::Enum,
        "defer" => TokenKind::Defer,
        "False" => TokenKind::False,
        "func" => TokenKind::Func,
        "for" => TokenKind::For,
        "if" => TokenKind::If,
        "impl" => TokenKind::Impl,
        "import" => TokenKind::Import,
        "in" => TokenKind::In,
        "interface" => TokenKind::Interface,
        "match" => TokenKind::Match,
        "None" => TokenKind::None,
        "or" => TokenKind::Or,
        "package" => TokenKind::Package,
        "pub" => TokenKind::Pub,
        "return" => TokenKind::Return,
        "struct" => TokenKind::Struct,
        "this" => TokenKind::This,
        "True" => TokenKind::True,
        "var" => TokenKind::Var,
        "while" => TokenKind::While,
        _ => TokenKind::Identifier
    }
}
