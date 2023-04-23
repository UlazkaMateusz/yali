use std::collections::HashMap;

use anyhow::Result;
use once_cell::unsync::Lazy;

const KEYWORDS: Lazy<HashMap<&str, TokenType>> = Lazy::new(|| {
    HashMap::from([
        ("and", TokenType::AND),
        ("class", TokenType::CLASS),
        ("else", TokenType::ELSE),
        ("false", TokenType::FALSE),
        ("for", TokenType::FOR),
        ("fun", TokenType::FUN),
        ("if", TokenType::IF),
        ("nil", TokenType::NIL),
        ("or", TokenType::OR),
        ("print", TokenType::PRINT),
        ("return", TokenType::RETURN),
        ("super", TokenType::SUPER),
        ("this", TokenType::THIS),
        ("true", TokenType::TRUE),
        ("var", TokenType::VAR),
        ("while", TokenType::WHILE),
    ])
});

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    line: i32,
    r#type: TokenType,
    text: String,
}

struct Position {
    line: i32,
    start: usize,
    current: usize,
}

pub struct Scanner<'a> {
    source: &'a str,
    position: Position,
    tokens: Vec<Token>,
}

impl Position {
    fn new() -> Self {
        Position {
            line: 1,
            start: 0,
            current: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    IDENTIFIER,
    STRING(String),
    NUMBER(f64),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl Scanner<'_> {
    pub fn new<'a>(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            position: Position::new(),
            tokens: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>> {
        while !self.is_at_end() {
            self.position.start = self.position.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            line: self.position.line,
            r#type: TokenType::EOF,
            text: String::new(),
        });

        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.position.current >= self.source.len()
    }

    fn peek_next(&self) -> char {
        if self.position.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.position.current + 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.position.current).unwrap()
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.position.current).unwrap();
        self.position.current += 1;

        char
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.substring(self.position.start..self.position.current);
        self.tokens.push(Token {
            line: self.position.line,
            r#type: token_type,
            text,
        });
    }

    fn substring(&self, range: std::ops::Range<usize>) -> String {
        String::from(&self.source[range])
    }

    fn is_alpha_or_underscore(&self, c: char) -> bool {
        c == '_' || c.is_ascii_alphabetic()
    }

    fn is_alphanumeric_or_underscore(&self, c: char) -> bool {
        c == '_' || c.is_ascii_alphanumeric()
    }

    fn scan_token(&mut self) -> Result<()> {
        let mut had_error = false;
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let token_type = if self.peek() == '=' {
                    self.position.current += 1;
                    TokenType::BangEqual
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.peek() == '=' {
                    self.position.current += 1;
                    TokenType::EqualEqual
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.peek() == '=' {
                    self.position.current += 1;

                    TokenType::LessEqual
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.peek() == '=' {
                    self.position.current += 1;
                    TokenType::GreaterEqual
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.peek() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.position.line += 1,
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.position.line += 1;
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    crate::error(self.position.line, "Unterminated string.");
                    return Err(anyhow::anyhow!("Unterminated string."));
                }

                // Consume closing "
                self.advance();

                let value = self.substring(self.position.start + 1..self.position.current - 1);
                self.add_token(TokenType::STRING(value));
            }
            '0'..='9' => {
                while self.peek().is_ascii_digit() {
                    self.advance();
                }

                if self.peek() == '.' && self.peek_next().is_ascii_digit() {
                    while self.peek().is_ascii_digit() {
                        self.advance();
                    }
                }

                let value = self
                    .substring(self.position.start..self.position.current)
                    .parse()?;

                self.add_token(TokenType::NUMBER(value));
            }
            char => {
                if self.is_alpha_or_underscore(char) {
                    self.identifier();
                } else {
                    had_error = true;
                    crate::error(self.position.line, &format!("Unexpected character {char}"))
                }
            }
        }

        if had_error {
            return Err(anyhow::anyhow!("Error occurred"));
        }

        Ok(())
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric_or_underscore(self.peek()) {
            self.advance();
        }

        let text = self.substring(self.position.start..self.position.current);
        if let Some(token_type) = KEYWORDS.get(&*text) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(TokenType::IDENTIFIER);
        }
    }
}
