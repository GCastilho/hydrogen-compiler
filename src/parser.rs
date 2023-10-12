use crate::{grammar::Statement, token::Token};
use std::{collections::HashMap, iter::Peekable, mem};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AstParserError {
    #[error("Unexpected end of token stream")]
    TokenIsNone,

    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token),

    #[error("Expected token {0:?}")]
    ExpectedToken(Token),

    #[error("Identifier already used {0}")]
    IdentifierAlreadyUsed(String),
}

#[derive(Debug)]
pub struct Program(Vec<Statement>);

impl Program {
    fn try_from_iter<I: Iterator<Item = Token>>(
        mut iter: Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let mut statements = vec![];
        while iter.peek().is_some() {
            statements.push(Statement::try_from_iter(&mut iter)?);
        }
        Ok(Self(statements))
    }

    pub fn to_asm(&self) -> String {
        let statements = self.0.iter().map(|s| s.to_asm()).collect::<String>();
        format!("global _start\n\n_start:\n{statements}")
    }
}

pub trait TreeParser: Sized {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError>;

    fn to_asm(&self) -> String;
}

pub trait TokenIterator
where
    Self: Sized,
    Self: Iterator<Item = Token>,
{
    fn next_token(&mut self) -> Result<Token, AstParserError>;
    fn peek_token(&mut self) -> Result<&Token, AstParserError>;
    fn expect_token(&mut self, token: Token) -> Result<Token, AstParserError>;
}

impl<I: Iterator<Item = Token>> TokenIterator for Peekable<I> {
    fn next_token(&mut self) -> Result<Token, AstParserError> {
        self.next().ok_or(AstParserError::TokenIsNone)
    }

    fn peek_token(&mut self) -> Result<&Token, AstParserError> {
        self.peek().ok_or(AstParserError::TokenIsNone)
    }

    fn expect_token(&mut self, token: Token) -> Result<Token, AstParserError> {
        let token_received = self.next_token()?;
        if mem::discriminant(&token) == mem::discriminant(&token_received) {
            Ok(token_received)
        } else {
            Err(AstParserError::ExpectedToken(token))
        }
    }
}

pub trait Parser
where
    Self: Sized,
    Self: Iterator<Item = Token>,
{
    fn parse(self) -> Result<Program, AstParserError>;
}

impl<I: Iterator<Item = Token>> Parser for I {
    fn parse(self) -> Result<Program, AstParserError>
    where
        Self: Sized,
        Self: Iterator<Item = Token>,
    {
        Program::try_from_iter(self.peekable())
    }
}

struct StackMetadata {
    stack_location: usize,
}

pub struct StackVarIdxMap {
    stack_size: usize,
    ident_stack_pos: HashMap<String, StackMetadata>,
}

impl StackVarIdxMap {
    pub fn new() -> Self {
        Self {
            stack_size: 0,
            ident_stack_pos: HashMap::new(),
        }
    }

    pub fn push(&mut self, reg: &str) -> String {
        self.stack_size += 1;
        format!("push {reg}\n")
    }

    pub fn pop(&mut self, reg: &str) -> String {
        self.stack_size -= 1;
        format!("pop {reg}\n")
    }

    pub fn contains_ident(&self, key: &str) -> bool {
        self.ident_stack_pos.contains_key(key)
    }

    pub fn insert(&mut self, ident: &str) {
        self.ident_stack_pos.insert(
            ident.to_string(),
            StackMetadata {
                stack_location: self.stack_size,
            },
        );
    }
}
