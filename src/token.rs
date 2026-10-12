#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: String,
}
impl Token {
    pub fn new(token_type: TokenType, line: usize, lexeme: String) -> Self {
        Self {
            line,
            token_type,
            lexeme,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
