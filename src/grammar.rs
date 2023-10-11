use crate::{
    parser::{AstParserError, TokenIterator, TreeParser},
    token::Token,
};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Statement {
    Exit(Expr),
    Let { ident: String, expr: Expr },
}

impl TreeParser for Statement {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let token = iter.next_token()?;
        let statement = match token {
            Token::Let => {
                let ident = match iter.next_token()? {
                    Token::Ident(ident) => ident,
                    _ => return Err(AstParserError::ExpectedToken(Token::Ident("ident".into()))),
                };
                iter.expect_token(Token::Eq)?;
                let expr = Expr::try_from_iter(iter)?;
                Ok(Self::Let { ident, expr })
            }
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
        match self {
            Statement::Exit(expr) => {
                let expr = expr.to_asm();
                format!(
                    "\
                    {expr}\
                    \x20\x20mov rax, 60\n\
                    \x20\x20pop rdi\n\
                    \x20\x20syscall\n\
                    "
                )
            }
            Statement::Let { ident, expr } => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    I64(i64),
    Ident(String),
}

impl TreeParser for Expr {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError> {
        let token = iter.next_token()?;
        let expr = match token {
            Token::I64Literal(i64) => Self::I64(i64),
            Token::Ident(ident) => Self::Ident(ident),
            _ => return Err(AstParserError::UnexpectedToken(token)),
        };
        Ok(expr)
    }

    fn to_asm(&self) -> String {
        match self {
            Expr::I64(i64) => {
                format!(
                    "\
                    \x20\x20mov rax, {i64}\n\
                    \x20\x20push rax\n\
                    "
                )
            }
            Expr::Ident(ident) => todo!(),
        }
    }
}
