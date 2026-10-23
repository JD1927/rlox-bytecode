use crate::{
    chunk::{Chunk, OpCode},
    scanner::Scanner,
    token::{Token, TokenType},
};

#[derive(Default)]
pub struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
    scanner: Scanner<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self {
            parser: Parser::default(),
            scanner: Scanner::default(),
        }
    }

    pub fn compile(&mut self, source: &'a str, chunk: &mut Chunk) -> bool {
        let scanner = Scanner::new(source);

        self.parser.had_error = false;
        self.parser.panic_mode = false;
        self.scanner = scanner;

        self.advance();

        self.expression();

        self.consume(TokenType::Eof, "Expect end of expression.");

        self.end_compiler(chunk);

        !self.parser.had_error
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current;

        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.is_not_error() {
                break;
            }
            self.error_at_current(self.parser.current.lexeme);
        }
    }

    fn expression(&self) {
        todo!()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.has_same_token_type(token_type) {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn emit_byte(&self, byte: u8, chunk: &mut Chunk) {
        chunk.write(byte, self.parser.previous.line);
    }

    fn emit_bytes(&self, byte1: u8, byte2: u8, chunk: &mut Chunk) {
        self.emit_byte(byte1, chunk);
        self.emit_byte(byte2, chunk);
    }

    fn end_compiler(&self, chunk: &mut Chunk) {
        self.emit_return(chunk)
    }

    fn emit_return(&self, chunk: &mut Chunk) {
        self.emit_byte(OpCode::Return.into(), chunk);
    }

    fn error_at_current(&mut self, message: &str) {
        let token = self.parser.current;
        self.error_at(token, message);
    }

    fn error(&mut self, message: &str) {
        let token = self.parser.previous;
        self.error_at(token, message);
    }

    fn error_at(&mut self, token: Token<'a>, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);
        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => {}
            _ => eprint!(" at '{}'", token.lexeme),
        }
        eprintln!(": {message}");
        self.parser.had_error = true;
    }
}
