use std::{collections::HashMap, sync::LazyLock};

use crate::{
    compiler::{Compiler, binary, grouping, number, unary},
    token::TokenType,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > >= <=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

pub type ParseFn = fn(&mut Compiler<'_>);

#[derive(Clone, Copy, Debug)]
pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

impl Precedence {
    // The next-higher precedence level. Saturates at `Primary`.
    pub fn next(self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary, // saturate, don't panic
        }
    }
}

impl ParseRule {
    pub const EMPTY: ParseRule = ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    };
    pub fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

pub static RULES: LazyLock<HashMap<TokenType, ParseRule>> = LazyLock::new(|| {
    HashMap::from([
        (
            TokenType::LeftParen,
            ParseRule::new(Some(grouping), None, Precedence::None),
        ),
        (
            TokenType::Minus,
            ParseRule::new(Some(unary), Some(binary), Precedence::Term),
        ),
        (
            TokenType::Plus,
            ParseRule::new(None, Some(binary), Precedence::Term),
        ),
        (
            TokenType::Slash,
            ParseRule::new(None, Some(binary), Precedence::Factor),
        ),
        (
            TokenType::Star,
            ParseRule::new(None, Some(binary), Precedence::Factor),
        ),
        (
            TokenType::Number,
            ParseRule::new(Some(number), None, Precedence::None),
        ),
    ])
});
