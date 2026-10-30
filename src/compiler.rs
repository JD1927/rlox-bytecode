use crate::{
    chunk::{Chunk, OpCode},
    precedence::{ParseRule, Precedence, RULES},
    scanner::Scanner,
    token::{Token, TokenType},
    value::Value,
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
    chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            parser: Parser::default(),
            scanner: Scanner::default(),
            chunk,
        }
    }

    pub fn compile(&mut self, source: &'a str) -> bool {
        let scanner = Scanner::new(source);

        self.parser.had_error = false;
        self.parser.panic_mode = false;
        self.scanner = scanner;

        self.advance();

        self.expression();

        self.consume(TokenType::Eof, "Expect end of expression.");

        self.end_compiler();

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

    pub fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn number(&mut self) {
        let value: Value = self.parser.previous.lexeme.parse().unwrap_or(0.0);
        self.emit_constant(value);
    }

    pub fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add.into()),
            TokenType::Minus => self.emit_byte(OpCode::Subtract.into()),
            TokenType::Star => self.emit_byte(OpCode::Multiply.into()),
            TokenType::Slash => self.emit_byte(OpCode::Divide.into()),
            _ => unreachable!(),
        }
    }

    pub fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        // Compile the operand.
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate.into()),
            TokenType::Bang => unimplemented!(),
            _ => unreachable!(), // Unreachable.
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let parse_rule = self.get_rule(self.parser.previous.token_type);

        let Some(prefix_rule_fn) = parse_rule.prefix else {
            self.error("Expect expression.");
            return;
        };

        prefix_rule_fn(self);

        while precedence <= self.get_rule(self.parser.current.token_type).precedence {
            self.advance();
            // The book would call a NULL function pointer here; Rust can't.
            let Some(infix_rule_fn) = self.get_rule(self.parser.previous.token_type).infix else {
                break;
            };
            infix_rule_fn(self);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.has_same_token_type(token_type) {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn end_compiler(&mut self) {
        self.emit_return()
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return.into());
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant.into(), constant);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.parser.previous.line);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        match constant > u8::MAX as usize {
            true => {
                self.error("Too many constants in one chunk.");
                0
            }
            false => constant as u8,
        }
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

    fn get_rule(&mut self, operator_type: TokenType) -> ParseRule {
        RULES
            .get(&operator_type)
            .copied()
            .unwrap_or(ParseRule::EMPTY)
    }
}

pub fn grouping(compiler: &mut Compiler<'_>) {
    compiler.grouping()
}
pub fn unary(compiler: &mut Compiler<'_>) {
    compiler.unary()
}
pub fn binary(compiler: &mut Compiler<'_>) {
    compiler.binary()
}
pub fn number(compiler: &mut Compiler<'_>) {
    compiler.number()
}
