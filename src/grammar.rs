use crate::{
    parser::{AstParserError, TokenIterator, TreeParser},
    token::Token,
};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Statement {
    Exit(Expr),
}

impl TreeParser for Statement {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let token = iter.next_token()?;
        match token {
            Token::Exit => Ok(Self::Exit(Expr::try_from_iter(iter)?)),
            _ => Err(AstParserError::UnexpectedToken(token)),
        }
    }
}

#[derive(Debug)]
pub struct Expr(Node);

impl TreeParser for Expr {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let node = Node::try_from_iter(iter)?;
        let token = iter.next_token()?;
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

impl TreeParser for Node {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let token = iter.next_token()?;
        match token {
            Token::I64Literal(i_64) => Ok(Self::I64(i_64)),
            _ => Err(AstParserError::UnexpectedToken(token)),
        }
    }
}
