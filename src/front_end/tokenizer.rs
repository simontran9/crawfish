use crate::front_end::token::{Span, Token, TokenKind};
use std::{iter::Peekable, str::CharIndices};

// TODO: maybe rename to syntax error?
#[derive(Debug)]
pub enum TokenizerError {
    InvalidCharacter,
}

/// Tokenizer
pub struct Tokenizer<'a> {
    source: &'a str,
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
        while let Some((start, c)) = self.chars.next() {
            match c {
                c if c.is_whitespace() => continue,
                '(' => {
                    return Ok(Token::new(
                        TokenKind::LeftCircleBracket,
                        Span::new(start, start + c.len_utf8()),
                    ))
                }
                ')' => {
                    return Ok(Token::new(
                        TokenKind::RightCircleBracket,
                        Span::new(start, start + c.len_utf8()),
                    ))
                }
                '{' => {
                    return Ok(Token::new(
                        TokenKind::LeftCurlyBracket,
                        Span::new(start, start + c.len_utf8()),
                    ))
                }
                '[' => {
                    return Ok(Token::new(
                        TokenKind::LeftSquareBracket,
                        Span::new(start, start + c.len_utf8()),
                    ))
                }
                ']' => {
                    return Ok(Token::new(
                        TokenKind::RightSquareBracket,
                        Span::new(start, start + c.len_utf8()),
                    ))
                }
                '&' => {
                        if let Some(&(eq_idx, '=')) = self.chars.peek() {
                            self.chars.next();
                            return Ok(Token::new(TokenKind::AmpersandEqual, Span::new(start, eq_idx + 1)));
                        }
                        return Ok(Token::new(TokenKind::Ampersand, Span::new(start, start + c.len_utf8())));
                    }
                '~' => return Ok(Token::new(TokenKind::Tilde, Span::new(start, start + c.len_utf8()))),
                '|' => {
                        if let Some(&(eq_idx, '=')) = self.chars.peek() {
                            self.chars.next();
                            return Ok(Token::new(TokenKind::PipeEqual, Span::new(start, eq_idx + 1)));
                        }
                        return Ok(Token::new(TokenKind::Pipe, Span::new(start, start + c.len_utf8())));
                    }
                '^' => {
                        if let Some(&(eq_idx, '=')) = self.chars.peek() {
                            self.chars.next();
                            return Ok(Token::new(TokenKind::CaretEqual, Span::new(start, eq_idx + 1)));
                        }
                        return Ok(Token::new(TokenKind::Caret, Span::new(start, start + c.len_utf8())));
                    }
                ':' => return Ok(Token::new(TokenKind::Colon, Span::new(start, start + c.len_utf8()))),
                ';' => return Ok(Token::new(TokenKind::Semicolon, Span::new(start, start + c.len_utf8()))),
                '.' => {
                    if let Some(&(dot_idx, '.')) = self.chars.peek() {
                        self.chars.next();
                        if let Some(&(eq_idx, '=')) = self.chars.peek() {
                            self.chars.next();
                            return Ok(Token::new(TokenKind::EllipsisEqual, Span::new(start, eq_idx + 1)));
                        }
                        return Ok(Token::new(TokenKind::Ellipsis, Span::new(start, dot_idx + 1)));
                    }
                    return Ok(Token::new(TokenKind::Dot, Span::new(start, start + c.len_utf8())));
                }
                ',' => return Ok(Token::new(TokenKind::Comma, Span::new(start, start + c.len_utf8()))),
                '=' => {
                    if let Some(&(second_eq_idx, '=')) = self.chars.peek() {
                        self.chars.next();
                        return Ok(Token::new(TokenKind::EqualEqual, Span::new(start, second_eq_idx + 1)));
                    }
                    if let Some(&(gt_idx, '>')) = self.chars.peek() {
                        self.chars.next();
                        return Ok(Token::new(TokenKind::FatArrow, Span::new(start, gt_idx + 1)));
                    }
                    return Ok(Token::new(TokenKind::Equal, Span::new(start, start + c.len_utf8())));
                }
                '!' => {
                        if let Some(&(eq_idx, '=')) = self.chars.peek() {
                            self.chars.next();
                            return Ok(Token::new(TokenKind::BangEqual, Span::new(start, eq_idx + 1)));
                        }
                        return Ok(Token::new(TokenKind::Bang, Span::new(start, start + c.len_utf8())));
                    }
                _ => return Err(TokenizerError::InvalidCharacter)
            }
        }
        let len = self.source.len();
        Ok(Token::new(
            TokenKind::EOF,
            Span::new(len, len),
        ))
    }
}


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

// reads the supposed identifier
// fn read_identifier(self: *Scanner) []const u8 {
//     const initial_index = self.current_index;
//     while (std.ascii.isAlphabetic(self.peek()) or std.ascii.isDigit(self.peek()) or self.peek() == '_') {
//         self.read();
//     }
//     return self.source[initial_index .. self.current_index + 1];
// }

// fn read_identifier(&mut Self) -> Token {
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
        _ => TokenKind::Identifier,
    }
}
