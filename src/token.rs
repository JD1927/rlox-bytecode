#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: &'a str,
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Token {
            token_type: TokenType::Nil,
            line: 0,
            lexeme: "",
        }
    }
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, line: usize, lexeme: &'a str) -> Self {
        Self {
            line,
            token_type,
            lexeme,
        }
    }

    pub fn is_not_error(&self) -> bool {
        self.token_type != TokenType::Error
    }

    pub fn has_same_token_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character Tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    Semicolon,
    Slash,
    Star,
    Question,

    // One or Two Character Tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Break,

    // Errors
    Error,

    // End of line
    Eof,
}
