//! An arithmetic expression calculator: lexer → parser → evaluator.

pub mod eval;
pub mod lexer;
pub mod parser;

use std::fmt;

/// A lexical token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Equals,
}

/// A binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// A parsed expression tree.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Var(String),
    Neg(Box<Expr>),
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// Everything that can go wrong.
#[derive(Debug, Clone, PartialEq)]
pub enum CalcError {
    UnexpectedChar(char),
    UnexpectedToken(Token),
    UnexpectedEnd,
    UnknownVariable(String),
    DivisionByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!("milestone 4: write a friendly message for each variant")
    }
}

impl std::error::Error for CalcError {}
