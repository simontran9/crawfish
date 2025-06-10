//! Tokenizer that turns source code into tokens
use crate::front_end::token::{Span, Token, TokenKind};
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    UnrecognizedCharacter(char),
    EmptyChar,
    UnterminatedChar,
    InvalidEscSeqChar,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizerError::UnrecognizedCharacter(c) => {
                write!(f, "Unrecognized character: '{}'", c)
            }
            TokenizerError::EmptyChar => write!(f, "Empty character literal"),
            TokenizerError::UnterminatedChar => write!(f, "Unterminated character literal"),
            TokenizerError::InvalidEscSeqChar => {
                write!(f, "Invalid escape sequence in character literal")
            }
        }
    }
}

impl Error for TokenizerError {}

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
        // Use an iterative loop instead of recursion to handle comments
        loop {
            self.skip_whitespace();

            let Some((start, c)) = self.chars.next() else {
                return Ok(Token::new(
                    TokenKind::EOF,
                    Span::new(self.source.len(), self.source.len()),
                ));
            };

            match c {
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
                '}' => {
                    return Ok(Token::new(
                        TokenKind::RightCurlyBracket,
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
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::AmpersandEqual, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Ampersand,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                '~' => {
                    return Ok(Token::new(
                        TokenKind::Tilde,
                        Span::new(start, start + c.len_utf8()),
                    ))
                }
                '|' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::PipeEqual, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Pipe,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                '^' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::CaretEqual, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Caret,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                ':' => {
                    if let Some(end) = self.match_next(':') {
                        return Ok(Token::new(TokenKind::DoubleColon, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Colon,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                ';' => {
                    return Ok(Token::new(
                        TokenKind::Semicolon,
                        Span::new(start, start + c.len_utf8()),
                    ))
                }
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
                    return Ok(Token::new(
                        TokenKind::Dot,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                ',' => {
                    return Ok(Token::new(
                        TokenKind::Comma,
                        Span::new(start, start + c.len_utf8()),
                    ))
                }
                '=' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::EqualEqual, Span::new(start, end)));
                    }
                    if let Some(end) = self.match_next('>') {
                        return Ok(Token::new(TokenKind::FatArrow, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Equal,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                '!' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::BangEqual, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Bang,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                '+' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::PlusEqual, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Plus,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                '-' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::MinusEqual, Span::new(start, end)));
                    }
                    if let Some(end) = self.match_next('>') {
                        return Ok(Token::new(TokenKind::SkinnyArrow, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Minus,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                '*' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::AsteriskEqual, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Asterisk,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                '/' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::SlashEqual, Span::new(start, end)));
                    }
                    if let Some(&(_, '/')) = self.chars.peek() {
                        self.chars.next();
                        self.skip_comment();
                        // Continue the loop instead of recursing
                        continue;
                    }
                    return Ok(Token::new(
                        TokenKind::Slash,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                '%' => {
                    if let Some(end) = self.match_next('=') {
                        return Ok(Token::new(TokenKind::PercentEqual, Span::new(start, end)));
                    }
                    return Ok(Token::new(
                        TokenKind::Percent,
                        Span::new(start, start + c.len_utf8()),
                    ));
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
                    return Ok(Token::new(
                        TokenKind::RightAngleBracket,
                        Span::new(start, start + c.len_utf8()),
                    ));
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
                    return Ok(Token::new(
                        TokenKind::LeftAngleBracket,
                        Span::new(start, start + c.len_utf8()),
                    ));
                }
                c if c.is_alphabetic() || c == '_' => {
                    let end = self.read_lexeme(start);
                    return Ok(Token::new(
                        Token::lexeme_token_kind(&self.source[start..end]),
                        Span::new(start, end),
                    ));
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
                        Some((end, '\'')) => {
                            return Ok(Token::new(
                                TokenKind::CharLiteral,
                                Span::new(start, end + '\''.len_utf8()),
                            ))
                        }
                        Some(_) => return Err(TokenizerError::UnterminatedChar),
                        None => return Err(TokenizerError::UnterminatedChar),
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
                    return Ok(Token::new(TokenKind::IntegerLiteral, Span::new(start, end)));
                }
                _ => return Err(TokenizerError::UnrecognizedCharacter(c)),
            }
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
        matches!(c, 'n' | 'r' | 't' | '0' | '\\' | '\'' | '\"')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_many_consecutive_comments() {
        // Test with many consecutive comment lines to ensure no stack overflow
        let mut source = String::new();
        for i in 0..10000 {
            source.push_str(&format!("// Comment line {}\n", i));
        }
        source.push_str("var x = 42;");

        let mut tokenizer = Tokenizer::new(&source);

        // Should skip all comments and find the variable declaration
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Var);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Identifier);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Equal);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Semicolon);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::EOF);
    }

    #[test]
    fn test_pathological_comment_input() {
        // Test with alternating comments and whitespace
        let mut source = String::new();
        for i in 0..5000 {
            source.push_str(&format!("   // Comment {}\n   ", i));
        }
        source.push_str("struct Test;");

        let mut tokenizer = Tokenizer::new(&source);

        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Struct);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Identifier);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Semicolon);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::EOF);
    }

    #[test]
    fn test_comment_at_eof() {
        let source = "var x // comment at end";
        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Var);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Identifier);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::EOF);
    }

    #[test]
    fn test_nested_comment_structure() {
        // Test structure that could cause issues with naive recursive approach
        let source = r#"
            var a = 1; // comment 1
            // comment 2
            var b = 2; // comment 3
            // comment 4
            // comment 5
            var c = 3;
        "#;

        let mut tokenizer = Tokenizer::new(source);

        // Should correctly parse all variable declarations
        let mut var_count = 0;
        loop {
            let token = tokenizer.next().unwrap();
            if token.kind == TokenKind::Var {
                var_count += 1;
            } else if token.kind == TokenKind::EOF {
                break;
            }
        }

        assert_eq!(var_count, 3);
    }

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

    #[test]
    fn test_single_char_tokens() {
        let source = r"(){}[]~;.,";
        let mut tokenizer = Tokenizer::new(source);
        let kinds = [
            TokenKind::LeftCircleBracket,
            TokenKind::RightCircleBracket,
            TokenKind::LeftCurlyBracket,
            TokenKind::RightCurlyBracket,
            TokenKind::LeftSquareBracket,
            TokenKind::RightSquareBracket,
            TokenKind::Tilde,
            TokenKind::Semicolon,
            TokenKind::Dot,
            TokenKind::Comma,
        ];

        for kind in kinds {
            let token = tokenizer.next().unwrap();
            assert_eq!(token.kind, kind);
        }
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::EOF);
    }

    #[test]
    fn test_multi_char_operators() {
        let source = r"& &= | |= ^ ^= :: : .. ..= == => -> != + +=";
        let expected_kinds = [
            TokenKind::Ampersand,
            TokenKind::AmpersandEqual,
            TokenKind::Pipe,
            TokenKind::PipeEqual,
            TokenKind::Caret,
            TokenKind::CaretEqual,
            TokenKind::DoubleColon,
            TokenKind::Colon,
            TokenKind::Ellipsis,
            TokenKind::EllipsisEqual,
            TokenKind::EqualEqual,
            TokenKind::FatArrow,
            TokenKind::SkinnyArrow,
            TokenKind::BangEqual,
            TokenKind::Plus,
            TokenKind::PlusEqual,
        ];

        let mut tokenizer = Tokenizer::new(source);
        for kind in expected_kinds {
            let token = tokenizer.next().unwrap();
            assert_eq!(token.kind, kind);
        }
    }

    #[test]
    fn test_arithmetic_and_assignment() {
        let source = "+ += - -= * *= / /= % %=";
        let expected_kinds = [
            TokenKind::Plus,
            TokenKind::PlusEqual,
            TokenKind::Minus,
            TokenKind::MinusEqual,
            TokenKind::Asterisk,
            TokenKind::AsteriskEqual,
            TokenKind::Slash,
            TokenKind::SlashEqual,
            TokenKind::Percent,
            TokenKind::PercentEqual,
        ];

        let mut tokenizer = Tokenizer::new(source);
        for kind in expected_kinds {
            let token = tokenizer.next().unwrap();
            assert_eq!(token.kind, kind);
        }
    }

    #[test]
    fn test_comparisons_and_shift_operators() {
        let source = "< <= << <<= > >= >> >>=";
        let expected_kinds = [
            TokenKind::LeftAngleBracket,
            TokenKind::LeftAngleBracketEqual,
            TokenKind::LeftAngleBracketLeftAngleBracket,
            TokenKind::LeftAngleBracketLeftAngleBracketEqual,
            TokenKind::RightAngleBracket,
            TokenKind::RightAngleBracketEqual,
            TokenKind::RightAngleBracketRightAngleBracket,
            TokenKind::RightAngleBracketRightAngleBracketEqual,
        ];

        let mut tokenizer = Tokenizer::new(source);
        for kind in expected_kinds {
            let token = tokenizer.next().unwrap();
            assert_eq!(token.kind, kind);
        }
    }

    #[test]
    fn test_identifier_and_keywords() {
        let source = "foo bar struct if else while for";
        let mut tokenizer = Tokenizer::new(source);

        let expected_kinds = [
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Struct,
            TokenKind::If,
            TokenKind::Else,
            TokenKind::While,
            TokenKind::For,
        ];

        for kind in expected_kinds {
            let token = tokenizer.next().unwrap();
            assert_eq!(token.kind, kind);
        }
    }

    #[test]
    fn test_integer_and_float_literals() {
        let source = "42 3.14 0 10.0 1.";
        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::FloatLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::FloatLiteral);
        // `1.` is not a float since there's no digit after the `.`
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Dot);
    }

    #[test]
    fn test_comments_are_skipped() {
        let source = r"// this is a comment
    var";
        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next().unwrap();
        assert_eq!(token.kind, TokenKind::Var);
    }

    #[test]
    fn test_unrecognized_character() {
        let source = "@";
        let mut tokenizer = Tokenizer::new(source);
        assert_eq!(
            tokenizer.next(),
            Err(TokenizerError::UnrecognizedCharacter('@'))
        );
    }

    #[test]
    fn test_char_escape_sequences() {
        let source = "'\\\\'  '\\''  '\\\"'  '\\0'";
        let mut tokenizer = Tokenizer::new(source);

        for _ in 0..4 {
            let token = tokenizer.next().unwrap();
            assert_eq!(token.kind, TokenKind::CharLiteral);
        }
    }

    #[test]
    fn test_empty_source() {
        let source = "";
        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }

    #[test]
    fn test_only_whitespace() {
        let source = "   \t\n\r  ";
        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }

    #[test]
    fn test_only_comments() {
        let source = "// first comment\n// second comment";
        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next().unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }

    #[test]
    fn test_comment_without_newline() {
        let source = "var // comment at end of file";
        let mut tokenizer = Tokenizer::new(source);

        let token1 = tokenizer.next().unwrap();
        assert_eq!(token1.kind, TokenKind::Var);

        let token2 = tokenizer.next().unwrap();
        assert_eq!(token2.kind, TokenKind::EOF);
    }

    #[test]
    fn test_mixed_whitespace_and_tokens() {
        let source = "  \t var  \n  42  \r\n  ";
        let mut tokenizer = Tokenizer::new(source);

        let token1 = tokenizer.next().unwrap();
        assert_eq!(token1.kind, TokenKind::Var);

        let token2 = tokenizer.next().unwrap();
        assert_eq!(token2.kind, TokenKind::IntegerLiteral);

        let token3 = tokenizer.next().unwrap();
        assert_eq!(token3.kind, TokenKind::EOF);
    }

    #[test]
    fn test_unicode_identifiers() {
        let source = "café αβγ δεζ _underscore русский";
        let mut tokenizer = Tokenizer::new(source);

        let token1 = tokenizer.next().unwrap();
        assert_eq!(token1.kind, TokenKind::Identifier);
        assert_eq!(token1.lexeme(source), "café");

        let token2 = tokenizer.next().unwrap();
        assert_eq!(token2.kind, TokenKind::Identifier);
        assert_eq!(token2.lexeme(source), "αβγ");

        let token3 = tokenizer.next().unwrap();
        assert_eq!(token3.kind, TokenKind::Identifier);
        assert_eq!(token3.lexeme(source), "δεζ");

        let token4 = tokenizer.next().unwrap();
        assert_eq!(token4.kind, TokenKind::Identifier);
        assert_eq!(token4.lexeme(source), "_underscore");

        let token5 = tokenizer.next().unwrap();
        assert_eq!(token5.kind, TokenKind::Identifier);
        assert_eq!(token5.lexeme(source), "русский");

        let token6 = tokenizer.next().unwrap();
        assert_eq!(token6.kind, TokenKind::EOF);
    }

    #[test]
    fn test_mixed_ascii_unicode_identifiers() {
        let source = "hello世界 test123 _test_测试";
        let mut tokenizer = Tokenizer::new(source);

        let token1 = tokenizer.next().unwrap();
        assert_eq!(token1.kind, TokenKind::Identifier);
        assert_eq!(token1.lexeme(source), "hello世界");

        let token2 = tokenizer.next().unwrap();
        assert_eq!(token2.kind, TokenKind::Identifier);
        assert_eq!(token2.lexeme(source), "test123");

        let token3 = tokenizer.next().unwrap();
        assert_eq!(token3.kind, TokenKind::Identifier);
        assert_eq!(token3.lexeme(source), "_test_测试");
    }

    #[test]
    fn test_underscore_identifiers() {
        let source = "_ _foo foo_ _foo_bar_ __double__";
        let mut tokenizer = Tokenizer::new(source);

        for _ in 0..5 {
            let token = tokenizer.next().unwrap();
            assert_eq!(token.kind, TokenKind::Identifier);
        }

        let eof = tokenizer.next().unwrap();
        assert_eq!(eof.kind, TokenKind::EOF);
    }

    #[test]
    fn test_numbers_with_leading_zeros() {
        let source = "0 00 007 0123";
        let mut tokenizer = Tokenizer::new(source);

        for _ in 0..4 {
            let token = tokenizer.next().unwrap();
            assert_eq!(token.kind, TokenKind::IntegerLiteral);
        }
    }

    #[test]
    fn test_float_edge_cases() {
        let source = "0.0 00.00 .5 1.";
        let mut tokenizer = Tokenizer::new(source);

        // 0.0
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::FloatLiteral);
        // 00.00
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::FloatLiteral);
        // .5 - should be dot followed by integer (not a float in this implementation)
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Dot);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        // 1. - should be integer followed by dot
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Dot);
    }

    #[test]
    fn test_adjacent_tokens_no_whitespace() {
        let source = "(){},;";
        let mut tokenizer = Tokenizer::new(source);

        let expected = [
            TokenKind::LeftCircleBracket,
            TokenKind::RightCircleBracket,
            TokenKind::LeftCurlyBracket,
            TokenKind::RightCurlyBracket,
            TokenKind::Comma,
            TokenKind::Semicolon,
        ];

        for kind in expected {
            assert_eq!(tokenizer.next().unwrap().kind, kind);
        }
    }

    #[test]
    fn test_operator_disambiguation() {
        let source = "< <= << <<=";
        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::LeftAngleBracket);
        assert_eq!(
            tokenizer.next().unwrap().kind,
            TokenKind::LeftAngleBracketEqual
        );
        assert_eq!(
            tokenizer.next().unwrap().kind,
            TokenKind::LeftAngleBracketLeftAngleBracket
        );
        assert_eq!(
            tokenizer.next().unwrap().kind,
            TokenKind::LeftAngleBracketLeftAngleBracketEqual
        );
    }

    #[test]
    fn test_ellipsis_variations() {
        let source = ". .. ... ..= ...=";
        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Dot);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Ellipsis);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Ellipsis);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Dot);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::EllipsisEqual);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Ellipsis);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Dot);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Equal);
    }

    #[test]
    fn test_char_with_quote_escape() {
        let source = "'\\'' '\\\"'";
        let mut tokenizer = Tokenizer::new(source);

        let token1 = tokenizer.next().unwrap();
        assert_eq!(token1.kind, TokenKind::CharLiteral);
        assert_eq!(token1.lexeme(source), "'\\''");

        let token2 = tokenizer.next().unwrap();
        assert_eq!(token2.kind, TokenKind::CharLiteral);
        assert_eq!(token2.lexeme(source), "'\\\"'");
    }

    #[test]
    fn test_unterminated_char_at_eof() {
        let source = "'";
        let mut tokenizer = Tokenizer::new(source);
        assert_eq!(tokenizer.next(), Err(TokenizerError::UnterminatedChar));
    }

    #[test]
    fn test_unterminated_escape_at_eof() {
        let source = r"'\";
        let mut tokenizer = Tokenizer::new(source);
        assert_eq!(tokenizer.next(), Err(TokenizerError::UnterminatedChar));
    }

    #[test]
    fn test_span_accuracy() {
        let source = "hello world";
        let mut tokenizer = Tokenizer::new(source);

        let token1 = tokenizer.next().unwrap();
        assert_eq!(token1.lexeme(source), "hello");

        let token2 = tokenizer.next().unwrap();
        assert_eq!(token2.lexeme(source), "world");
    }

    #[test]
    fn test_multiple_consecutive_operators() {
        let source = "==>>>===";
        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::EqualEqual);
        assert_eq!(
            tokenizer.next().unwrap().kind,
            TokenKind::RightAngleBracketRightAngleBracket
        );
        assert_eq!(
            tokenizer.next().unwrap().kind,
            TokenKind::RightAngleBracketEqual
        );
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::EqualEqual);
    }

    #[test]
    fn test_various_unrecognized_characters() {
        let unrecognized_chars = ['@', '#', '$', '?', '`'];

        for &c in &unrecognized_chars {
            let source = c.to_string();
            let mut tokenizer = Tokenizer::new(&source);
            assert_eq!(
                tokenizer.next(),
                Err(TokenizerError::UnrecognizedCharacter(c))
            );
        }
    }

    #[test]
    fn test_bom_with_following_content() {
        let source = "\u{FEFF}var x = 42;";
        let mut tokenizer = Tokenizer::new(source);

        // BOM should be skipped, first token should be 'var'
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Var);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Identifier);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Equal);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_long_number_sequences() {
        let source = "123456789 987654321.123456789";
        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::FloatLiteral);
    }

    #[test]
    fn test_comment_after_various_tokens() {
        let source = "var // comment\n42 // another comment\n";
        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::Var);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::IntegerLiteral);
        assert_eq!(tokenizer.next().unwrap().kind, TokenKind::EOF);
    }
}
