use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    pub line: usize,
    pub start: usize,
    pub current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        // Checking for identifiers
        if c.is_ascii_alphabetic() || c == b'_' {
            return self.identifier();
        }

        // Checking for numbers
        if c.is_ascii_digit() {
            return self.number();
        }

        match c {
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b';' => self.make_token(TokenType::Semicolon),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b':' => self.make_token(TokenType::Colon),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),
            b'?' => self.make_token(TokenType::Question),
            b'!' => {
                if self.match_with(b'=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            b'=' => {
                if self.match_with(b'=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            b'<' => {
                if self.match_with(b'=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            b'>' => {
                if self.match_with(b'=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            b'"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn identifier(&mut self) -> Token<'a> {
        while self.peek().is_ascii_alphabetic() || self.peek().is_ascii_digit() {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        let lexeme = &self.source[self.start..self.current];

        match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            // The closing quote
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            // Consume the '.';
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token::new(
            token_type,
            self.line,
            &self.source[self.start..self.current],
        )
    }

    fn error_token(&self, error_message: &'static str) -> Token<'a> {
        Token::new(TokenType::Error, self.line, error_message)
    }

    fn advance(&mut self) -> u8 {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        c
    }

    fn match_with(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source.as_bytes()[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }
            let c = self.peek();
            match c {
                b' ' | b'\r' | b'\t' => self.advance(),
                b'\n' => {
                    self.line += 1;
                    self.advance()
                }
                b'/' => {
                    if self.peek_next() == b'/' {
                        // A comment goes until the end of the line
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                    break;
                }
                _ => break,
            };
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.source.as_bytes()[self.current]
    }

    fn peek_next(&self) -> u8 {
        match self.current + 1 >= self.source.len() {
            true => b'\0',
            false => self.source.as_bytes()[self.current + 1],
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
