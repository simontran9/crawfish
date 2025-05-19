// pub struct LexicalAnalyzer<'a> {
//     source: &'a str,
//     current_index: usize,
//     read_index: usize,
//     current_char: char,
// }

// impl<'a> Scanner<'a> {
//     const BOM: &str = "\u{FEFF}";
//     const ASCII_NULL: char = '\0';

//     /// Creates and returns a scanner object
//     pub fn new(source: &'a str) -> Self {
//         let start_index = if source.starts_with(Self::BOM) {
//             Self::BOM.len()
//         } else {
//             0
//         };

//         let mut char_iter = source[start_index..].char_indices();
//         let (offset, current_char) =
//             char_iter.next().unwrap_or((0, Self::ASCII_NULL));
//         let read_index = start_index + offset + current_char.len_utf8();

//         Scanner {
//             source,
//             current_index: start_index,
//             read_index,
//             current_char,
//         }
//     }

// Recturns the next token
// pub fn next(self: *Scanner) Token {
//     self.skip_whitespace();
//     var token: Token = undefined;
//     switch (self.current_char) {
//         '(' => token = Token.init(Token.TokenType.LeftCircleBrack, null),
//         ')' => token = Token.init(Token.TokenType.RightCircleBrack, null),
//         '{' => token = Token.init(Token.TokenType.LeftCurlyBrack, null),
//         '}' => token = Token.init(Token.TokenType.RightCurlyBrack, null),
//         '[' => token = Token.init(Token.TokenType.LeftSquareBrack, null),
//         ']' => token = Token.init(Token.TokenType.RightSquareBrack, null),
//         '?' => token = Token.init(Token.TokenType.QuestionMark, null),
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

// skips whitespace or \t and \r escape sequences
// fn skip_whitespace(self: *Scanner) void {
//     while (self.current_char == ' ' or self.current_char == '\n' or self.current_char == '\t' or self.current_char == '\r') {
//         self.read();
//     }
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
