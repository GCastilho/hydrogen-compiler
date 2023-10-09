use crate::{grammar::Statement, token::Token};
use std::iter::Peekable;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AstParserError {
    #[error("Unexpected end of token stream")]
    TokenIsNone,

    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token),
}

#[derive(Debug)]
pub struct Root(Statement);

impl Root {
    fn try_from_iter<I: Iterator<Item = Token>>(
        mut iter: Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let token = iter.peek_token()?;
        let node = match token {
            Token::Exit => Statement::try_from_iter(&mut iter)?,
            _ => return Err(AstParserError::UnexpectedToken(*token)),
        };
        Ok(Self(node))
    }
}

pub trait TreeParser: Sized {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError>;
}

pub trait TokenIterator
where
    Self: Sized,
    Self: Iterator<Item = Token>,
{
    fn next_token(&mut self) -> Result<Token, AstParserError>;
    fn peek_token(&mut self) -> Result<&Token, AstParserError>;
}

impl<I: Iterator<Item = Token>> TokenIterator for Peekable<I> {
    fn next_token(&mut self) -> Result<Token, AstParserError> {
        self.next().ok_or(AstParserError::TokenIsNone)
    }

    fn peek_token(&mut self) -> Result<&Token, AstParserError> {
        self.peek().ok_or(AstParserError::TokenIsNone)
    }
}

pub trait Parser
where
    Self: Sized,
    Self: Iterator<Item = Token>,
{
    fn parse(self) -> Result<Root, AstParserError>;
}

impl<I: Iterator<Item = Token>> Parser for I {
    fn parse(self) -> Result<Root, AstParserError>
    where
        Self: Sized,
        Self: Iterator<Item = Token>,
    {
        Root::try_from_iter(self.peekable())
    }
}
