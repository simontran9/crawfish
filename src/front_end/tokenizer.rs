use crate::front_end::token::{Span, Token, TokenKind};
use std::iter::Peekable;
use std::str::CharIndices;

// TODO: implement fmt::Display
#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    UnrecognizedCharacter(char),
    WindowsLineEndingDisallowed,
    EmptyChar,
    UnterminatedChar,
    InvalidEscSeqChar,
}

/// Tokenizer
/// - `source` is a string slice to the original source code
/// - `chars` is an iterator that returns `(index, character)` elements
///    and supports lookahead out of the box
pub struct Tokenizer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Tokenizer<'a> {
    const BOM: char = '\u{FEFF}';

    /// Returns a tokenizer iterator
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.char_indices().peekable();

        if let Some(&(_, c)) = chars.peek() {
            if c == Self::BOM {
                chars.next();
            }
        }

        Self { source, chars }
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
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::AmpersandEqual, Span::new(start, end)));
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
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::PipeEqual, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Pipe,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '^' => {
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::CaretEqual, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Caret,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            ':' => {
                if let Some(end) = self.match_next(':') {
                    return Ok(Token::new(TokenKind::DoubleColon, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Colon,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            ';' => Ok(Token::new(
                TokenKind::Semicolon,
                Span::new(start, start + c.len_utf8()),
            )),
            '.' => {
                if let Some(&(dot_idx, '.')) = self.chars.peek() {
                    self.chars.next();
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::EllipsisEqual, Span::new(start, end)));
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
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::EqualEqual, Span::new(start, end)));
                }
                if let Some(end) = self.match_next('>') {
                    return Ok(Token::new(TokenKind::FatArrow, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Equal,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '!' => {
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::BangEqual, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Bang,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '+' => {
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::PlusEqual, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Plus,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '-' => {
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::MinusEqual, Span::new(start, end)));
                }
                if let Some(end) = self.match_next('>') {
                    return Ok(Token::new(TokenKind::SkinnyArrow, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Minus,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '*' => {
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::AsteriskEqual, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Asterisk,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '/' => {
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::SlashEqual, Span::new(start, end)));
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
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(TokenKind::PercentEqual, Span::new(start, end)));
                }
                Ok(Token::new(
                    TokenKind::Percent,
                    Span::new(start, start + c.len_utf8()),
                ))
            }
            '>' => {
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(
                        TokenKind::RightAngleBracketEqual,
                        Span::new(start, end),
                    ));
                }
                if let Some(&(gt_idx, '>')) = self.chars.peek() {
                    self.chars.next();
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(
                            TokenKind::RightAngleBracketRightAngleBracketEqual,
                            Span::new(start, end),
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
                if let Some(end) = self.match_next('=') {
                    return Ok(Token::new(
                        TokenKind::LeftAngleBracketEqual,
                        Span::new(start, end),
                    ));
                }
                if let Some(&(gt_idx, '<')) = self.chars.peek() {
                    self.chars.next();
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(
                            TokenKind::LeftAngleBracketLeftAngleBracketEqual,
                            Span::new(start, end),
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
                Ok(Token::new(
                    Token::lexeme_token_kind(&self.source[start..end]),
                    Span::new(start, end),
                ))
            }
            '\'' => {
                match self.chars.next() {
                    Some((_, '\\')) => match self.chars.next() {
                        Some((_, c)) if Self::is_single_char_escape_sequence(c) => (),
                        Some(_) => return Err(TokenizerError::InvalidEscSeqChar),
                        None => return Err(TokenizerError::UnterminatedChar),
                    },
                    Some((_, c)) if c != '\'' => (),
                    Some(_) => return Err(TokenizerError::EmptyChar),
                    None => return Err(TokenizerError::UnterminatedChar),
                };

                match self.chars.next() {
                    Some((end, '\'')) => Ok(Token::new(
                        TokenKind::CharLiteral,
                        Span::new(start, end + '\''.len_utf8()),
                    )),
                    Some(_) => Err(TokenizerError::UnterminatedChar),
                    None => Err(TokenizerError::UnterminatedChar),
                }
            }
            '0'..='9' => {
                let end = self.read_number(start);
                if let Some(&(_, '.')) = self.chars.peek() {
                    // Save state, without having to modify the actual underlying iterator, via `clone()`,
                    // before consuming the dot to rewind as necessary
                    let mut chars_clone = self.chars.clone();
                    chars_clone.next();
                    if let Some(&(after_dot_idx, after_dot_char)) = chars_clone.peek() {
                        // Fraction detected, so we consume the dot on the actual underlying iterator,
                        // and read the rest to see if it is indeed a float
                        if after_dot_char.is_ascii_digit() {
                            self.chars.next();
                            let float_end = self.read_number(after_dot_idx);
                            return Ok(Token::new(
                                TokenKind::FloatLiteral,
                                Span::new(start, float_end),
                            ));
                        }
                    }
                    // If it wasn't a float, then we didn't modify the actual underlying iterator, so not an issue
                    // and we remain at the last number right before the dot
                }
                Ok(Token::new(TokenKind::IntegerLiteral, Span::new(start, end)))
            }
            '\r' => Err(TokenizerError::WindowsLineEndingDisallowed),
            _ => Err(TokenizerError::UnrecognizedCharacter(c)),
        }
    }

    fn skip_whitespace(&mut self) {
        self.skip_while(|c| c.is_whitespace());
    }

    fn skip_comment(&mut self) {
        self.skip_while(|c| c != '\n');
    }

    fn skip_while<F>(&mut self, predicate: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(&(_, c)) = self.chars.peek() {
            if !predicate(c) {
                break;
            }
            self.chars.next();
        }
    }

    fn match_next(&mut self, expected: char) -> Option<usize> {
        match self.chars.peek() {
            Some(&(i, c)) if c == expected => {
                self.chars.next();
                Some(i + expected.len_utf8())
            }
            _ => None,
        }
    }

    fn read_lexeme(&mut self, start: usize) -> usize {
        self.read_while(start, |c| c.is_alphanumeric() || c == '_')
    }

    fn read_number(&mut self, start: usize) -> usize {
        self.read_while(start, |c| c.is_ascii_digit())
    }

    fn read_while<F>(&mut self, start: usize, predicate: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut end = start;
        while let Some(&(i, c)) = self.chars.peek() {
            if predicate(c) {
                self.chars.next();
                // Exclusive end can be calculated by taking the last character's
                // index, and adding its length. We continuously calculate end
                // in here, rather than in the else branch, because otherwise,
                // the exlusive end would include the first rejected character.
                end = i + c.len_utf8();
            } else {
                break;
            }
        }
        end
    }

    fn is_single_char_escape_sequence(c: char) -> bool {
        match c {
            'n' | 'r' | 't' | '0' | '\\' | '\'' | '\"' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bom_is_skipped() {
        let source = "\u{FEFF}struct";
        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next().unwrap();
        assert_eq!(token.kind, TokenKind::Struct);
    }

    #[test]
    fn test_valid_operators() {
        let source = r"!=";
        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next().unwrap();
        assert_eq!(token.kind, TokenKind::BangEqual);
    }

    #[test]
    fn test_valid_char() {
        let source = r"'a' '\n' '\r' '\t' '\0' '\\'";
        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.next().unwrap().lexeme(source), r"'a'");
        assert_eq!(tokenizer.next().unwrap().lexeme(source), r"'\n'");
        assert_eq!(tokenizer.next().unwrap().lexeme(source), r"'\r'");
        assert_eq!(tokenizer.next().unwrap().lexeme(source), r"'\t'");
        assert_eq!(tokenizer.next().unwrap().lexeme(source), r"'\0'");
        assert_eq!(tokenizer.next().unwrap().lexeme(source), r"'\\'");
    }

    #[test]
    fn test_empty_char() {
        let source = r"''";
        let mut tokenizer = Tokenizer::new(source);
        assert_eq!(tokenizer.next(), Err(TokenizerError::EmptyChar));
    }

    #[test]
    fn test_invalid_single_char_esc_seq() {
        let source = r"'\a'";
        let mut tokenizer = Tokenizer::new(source);
        assert_eq!(tokenizer.next(), Err(TokenizerError::InvalidEscSeqChar));
    }

    #[test]
    fn test_more_than_one_char_in_char() {
        let source = r"'ab'";
        let mut tokenizer = Tokenizer::new(source);
        assert_eq!(tokenizer.next(), Err(TokenizerError::UnterminatedChar));
    }

    #[test]
    fn test_unterminated_char() {
        let source = r"'a";
        let mut tokenizer = Tokenizer::new(source);
        assert_eq!(tokenizer.next(), Err(TokenizerError::UnterminatedChar));
    }
}
