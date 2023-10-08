use crate::token::Token;
use std::iter::Peekable;
use thiserror::Error;

#[derive(Debug)]
pub struct Root(Statement);

impl Root {
    fn try_from_iter<I: Iterator<Item = Token>>(
        mut iter: Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let token = iter.peek().ok_or(AstParserError::TokenIsNone)?;
        let node = match token {
            Token::Exit => Statement::try_from_iter(&mut iter)?,
            _ => return Err(AstParserError::UnexpectedToken(*token)),
        };
        Ok(Self(node))
    }
}

#[derive(Debug)]
pub enum Statement {
    Exit(Expr),
}

impl Statement {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let token = iter.next().ok_or(AstParserError::TokenIsNone)?;
        match token {
            Token::Exit => Ok(Self::Exit(Expr::try_from_iter(iter)?)),
            _ => Err(AstParserError::UnexpectedToken(token)),
        }
    }
}

#[derive(Debug)]
pub struct Expr(Node);

impl Expr {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let node = Node::try_from_iter(iter)?;
        let token = iter.next().ok_or(AstParserError::TokenIsNone)?;
        if matches!(token, Token::Semi) {
            Ok(Self(node))
        } else {
            Err(AstParserError::UnexpectedToken(token))
        }
    }
}

#[derive(Debug)]
pub enum Node {
    I64(i64),
}

impl Node {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let token = iter.next().ok_or(AstParserError::TokenIsNone)?;
        match token {
            Token::I64Literal(i_64) => Ok(Self::I64(i_64)),
            _ => Err(AstParserError::UnexpectedToken(token)),
        }
    }
}

#[derive(Debug, Error)]
pub enum AstParserError {
    #[error("Unexpected end of token stream")]
    TokenIsNone,

    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token),
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
