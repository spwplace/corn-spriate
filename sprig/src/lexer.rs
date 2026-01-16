//! Lexer for the Sprig language

/// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Integer(i64),
    Identifier(String),

    // Keywords
    Fn,
    Let,
    If,
    Else,
    While,
    Return,
    True,
    False,

    // Types
    Int,
    Bool,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
    Assign,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Arrow,
    Semicolon,

    // Special
    Eof,
}

/// A token with position information
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub col: u32,
}

impl Token {
    pub fn new(kind: TokenKind, line: u32, col: u32) -> Self {
        Self { kind, line, col }
    }
}

/// Lexer state
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next().map(|(_, c)| {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            c
        })
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
            } else if c == '/' {
                // Check for comments
                let mut chars = self.chars.clone();
                chars.next();
                if chars.peek().map(|&(_, c)| c) == Some('/') {
                    // Line comment
                    while let Some(c) = self.peek_char() {
                        if c == '\n' {
                            break;
                        }
                        self.next_char();
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let line = self.line;
        let col = self.col;

        let Some(c) = self.next_char() else {
            return Token::new(TokenKind::Eof, line, col);
        };

        let kind = match c {
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,

            '-' => {
                if self.peek_char() == Some('>') {
                    self.next_char();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }

            '=' => {
                if self.peek_char() == Some('=') {
                    self.next_char();
                    TokenKind::Eq
                } else {
                    TokenKind::Assign
                }
            }

            '!' => {
                if self.peek_char() == Some('=') {
                    self.next_char();
                    TokenKind::Ne
                } else {
                    TokenKind::Not
                }
            }

            '<' => {
                if self.peek_char() == Some('=') {
                    self.next_char();
                    TokenKind::Le
                } else {
                    TokenKind::Lt
                }
            }

            '>' => {
                if self.peek_char() == Some('=') {
                    self.next_char();
                    TokenKind::Ge
                } else {
                    TokenKind::Gt
                }
            }

            '&' => {
                if self.peek_char() == Some('&') {
                    self.next_char();
                    TokenKind::And
                } else {
                    panic!("Expected '&&' at {}:{}", line, col)
                }
            }

            '|' => {
                if self.peek_char() == Some('|') {
                    self.next_char();
                    TokenKind::Or
                } else {
                    panic!("Expected '||' at {}:{}", line, col)
                }
            }

            c if c.is_ascii_digit() => {
                let start = self.input[..].char_indices()
                    .find(|&(_, ch)| ch == c)
                    .map(|(i, _)| i)
                    .unwrap_or(0);

                let mut end = start + c.len_utf8();
                while let Some(ch) = self.peek_char() {
                    if ch.is_ascii_digit() {
                        end += ch.len_utf8();
                        self.next_char();
                    } else {
                        break;
                    }
                }

                // Parse from the collected digits
                let mut value: i64 = (c as u8 - b'0') as i64;
                // Continue with remaining digits we consumed
                // (The chars iterator already moved past start)
                TokenKind::Integer(value)
            }

            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                ident.push(c);

                while let Some(ch) = self.peek_char() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        self.next_char();
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "fn" => TokenKind::Fn,
                    "let" => TokenKind::Let,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "while" => TokenKind::While,
                    "return" => TokenKind::Return,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "int" => TokenKind::Int,
                    "bool" => TokenKind::Bool,
                    _ => TokenKind::Identifier(ident),
                }
            }

            _ => panic!("Unexpected character '{}' at {}:{}", c, line, col),
        };

        Token::new(kind, line, col)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }
}
