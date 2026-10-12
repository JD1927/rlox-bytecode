use crate::token::{self, Token, TokenType};

pub struct Scanner {
    pub source: Vec<char>,
    pub line: usize,
    pub start: usize,
    pub current: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect::<Vec<char>>(),
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        self.error_token("Unexpected character.")
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.line,
            self.source[self.start..self.current].iter().collect(),
        )
    }

    pub fn error_token(&self, error_message: &str) -> Token {
        Token::new(TokenType::Error, self.line, error_message.to_string())
    }
}
