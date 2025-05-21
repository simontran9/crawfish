use crate::front_end::token::{Span, Token, TokenKind};
use std::iter::Peekable;
use std::str::CharIndices;

// TODO: maybe rename to syntax error?
#[derive(Debug)]
pub enum TokenizerError {
    InvalidCharacter,
    WindowsLineEndingDisallowed,
}

/// Tokenizer
pub struct Tokenizer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

// TODO: handle const BOM: &str = "\u{FEFF}";
// TODO: implement regular string
// TODO: implement multi-line string
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
        self.skip_whitespace();

        let Some((start, c)) = self.chars.next() else {
            return Ok(Token::new(
                TokenKind::EOF,
                Span::new(self.source.len(), self.source.len()),
            ));
        };

        match c {
            '(' => Ok(Token::new(
                TokenKind::LeftCircleBracket,
                Span::new(start, start + c.len_utf8()),
            )),
            ')' => Ok(Token::new(
                TokenKind::RightCircleBracket,
                Span::new(start, start + c.len_utf8()),
            )),
            '{' => Ok(Token::new(
                TokenKind::LeftCurlyBracket,
                Span::new(start, start + c.len_utf8()),
            )),
            '[' => Ok(Token::new(
                TokenKind::LeftSquareBracket,
                Span::new(start, start + c.len_utf8()),
            )),
            ']' => Ok(Token::new(
                TokenKind::RightSquareBracket,
                Span::new(start, start + c.len_utf8()),
            )),
            '&' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::AmpersandEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Ampersand,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '~' => Ok(Token::new(
                TokenKind::Tilde,
                Span::new(start, start + c.len_utf8()),
            )),
            '|' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::PipeEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Pipe,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '^' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::CaretEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Caret,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            ':' => Ok(Token::new(
                TokenKind::Colon,
                Span::new(start, start + c.len_utf8()),
            )),
            ';' => Ok(Token::new(
                TokenKind::Semicolon,
                Span::new(start, start + c.len_utf8()),
            )),
            '.' => {
                if let Some(&(dot_idx, '.')) = self.chars.peek() {
                    self.chars.next();
                    if let Some(&(eql_idx, '=')) = self.chars.peek() {
                        self.chars.next();
                        return Ok(Token::new(
                            TokenKind::EllipsisEqual,
                            Span::new(start, eql_idx + '='.len_utf8()),
                        ));
                    }
                    return Ok(Token::new(
                        TokenKind::Ellipsis,
                        Span::new(start, dot_idx + 1),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Dot,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            ',' => Ok(Token::new(
                TokenKind::Comma,
                Span::new(start, start + c.len_utf8()),
            )),
            '=' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::EqualEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                if let Some(&(gt_idx, '>')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::FatArrow,
                        Span::new(start, gt_idx + '>'.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Equal,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '!' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::BangEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Bang,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '+' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::PlusEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Plus,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '-' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::MinusEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                if let Some(&(gt_idx, '>')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::SkinnyArrow,
                        Span::new(start, gt_idx + '>'.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Minus,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '*' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::AsteriskEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Asterisk,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '/' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::SlashEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                if let Some(&(_, '/')) = self.chars.peek() {
                    self.chars.next();
                    self.skip_comment();
                    return self.next();
                }
                Ok(Token::new(
                    TokenKind::Slash,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '%' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::PercentEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::Percent,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '>' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::RightAngleBracketEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                if let Some(&(gt_idx, '>')) = self.chars.peek() {
                    self.chars.next();
                    if let Some(&(eql_idx, '=')) = self.chars.peek() {
                        self.chars.next();
                        return Ok(Token::new(
                            TokenKind::RightAngleBracketRightAngleBracketEqual,
                            Span::new(start, eql_idx + '='.len_utf8()),
                        ));
                    }
                    return Ok(Token::new(
                        TokenKind::RightAngleBracketRightAngleBracket,
                        Span::new(start, gt_idx + '>'.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::RightAngleBracket,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '<' => {
                if let Some(&(eql_idx, '=')) = self.chars.peek() {
                    self.chars.next();
                    return Ok(Token::new(
                        TokenKind::LeftAngleBracketEqual,
                        Span::new(start, eql_idx + '='.len_utf8()),
                    ));
                }
                if let Some(&(gt_idx, '<')) = self.chars.peek() {
                    self.chars.next();
                    if let Some(&(eql_idx, '=')) = self.chars.peek() {
                        self.chars.next();
                        return Ok(Token::new(
                            TokenKind::LeftAngleBracketLeftAngleBracketEqual,
                            Span::new(start, eql_idx + '='.len_utf8()),
                        ));
                    }
                    return Ok(Token::new(
                        TokenKind::LeftAngleBracketLeftAngleBracket,
                        Span::new(start, gt_idx + '<'.len_utf8()),
                    ));
                }
                Ok(Token::new(
                    TokenKind::LeftAngleBracket,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let end = self.read_lexeme(start);
                println!("{}", &self.source[start..end].to_string());
                Ok(Token::new(
                    lexeme_token_kind(&self.source[start..end]),
                    Span::new(start, end),
                ))
            }
            '0'..='9' => {
                todo!();
            }
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
            //     }
            '\r' => Err(TokenizerError::WindowsLineEndingDisallowed),
            _ => Err(TokenizerError::InvalidCharacter),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&(_, c)) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.chars.next();
        }
    }

    fn skip_comment(&mut self) {
        while let Some(&(_, c)) = self.chars.peek() {
            if c == '\n' {
                break;
            }
            self.chars.next();
        }
    }

    fn read_lexeme(&mut self, start: usize) -> usize {
        let mut end = start;
        while let Some(&(i, c)) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.chars.next();
                end = i + c.len_utf8(); // for exclusive end
            } else {
                break;
            }
        }
        end
    }

    fn read_int(&mut self) {
        todo!();
    }

    fn read_float(&mut self) {
        todo!();
    }
}

fn lexeme_token_kind(ident: &str) -> TokenKind {
    match ident {
        "and" => TokenKind::And,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "const" => TokenKind::Const,
        "else" => TokenKind::Else,
        "enum" => TokenKind::Enum,
        "defer" => TokenKind::Defer,
        "False" => TokenKind::False,
        "func" => TokenKind::Func,
        "for" => TokenKind::For,
        "if" => TokenKind::If,
        "implements" => TokenKind::Implements,
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
