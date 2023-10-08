use crate::token::Token;
use thiserror::Error;

#[derive(Debug)]
pub struct Root(Statement);

impl Root {
    fn try_from_iter<I: Iterator<Item = Token>>(iter: I) -> Result<Self, AstParserError> {
        todo!()
    }
}

impl Root {
    // TODO remove
    pub fn new(statement: Statement) -> Self {
        Self(statement)
    }
}

#[derive(Debug)]
pub enum Statement {
    Exit(Expr),
}

#[derive(Debug)]
pub struct Expr(Node);

impl Expr {
    // TODO remove
    pub fn new(node: Node) -> Self {
        Self(node)
    }
}

#[derive(Debug)]
pub enum Node {
    I64(i64),
}

#[derive(Debug, Error)]
pub enum AstParserError {}

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
        Root::try_from_iter(self)
    }
}
