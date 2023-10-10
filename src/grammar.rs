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
        let statement = match token {
            Token::Exit => {
                iter.expect_token(Token::ParenOpen)?;
                let exit = Self::Exit(Expr::try_from_iter(iter)?);
                iter.expect_token(Token::ParenClose)?;
                Ok(exit)
            }
            _ => Err(AstParserError::UnexpectedToken(token)),
        }?;
        if iter.expect_token(Token::Semi).is_ok() {
            Ok(statement)
        } else {
            Err(AstParserError::ExpectedToken(Token::Semi))
        }
    }

    fn to_asm(&self) -> String {
        let expr = match self {
            Statement::Exit(expr) => expr.to_asm(),
        };
        format!(
            "\
            \x20\x20mov rax, 60\n\
            \x20\x20mov rdi, {expr}\n\
            \x20\x20syscall\n\
            "
        )
    }
}

#[derive(Debug)]
pub struct Expr(Node);

impl TreeParser for Expr {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        Ok(Self(Node::try_from_iter(iter)?))
    }

    fn to_asm(&self) -> String {
        self.0.to_asm()
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

    fn to_asm(&self) -> String {
        match self {
            Node::I64(i_64) => i_64.to_string(),
        }
    }
}
