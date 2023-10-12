use crate::{
    parser::{AsmStream, AstParserError, StackVarIdxMap, TokenIterator, TreeParser},
    token::Token,
};
use lazy_static::lazy_static;
use std::iter::Peekable;

lazy_static! {
    static ref STACK_IDX_TRACKER: StackVarIdxMap = StackVarIdxMap::new();
}

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
                if STACK_IDX_TRACKER.contains_ident(&ident) {
                    return Err(AstParserError::IdentifierAlreadyUsed(ident));
                }
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

    fn to_asm(&self, asm_stream: &mut AsmStream) {
        match self {
            Statement::Exit(expr) => {
                expr.to_asm(asm_stream);
                asm_stream.write_line("mov rax, 60");
                STACK_IDX_TRACKER.pop(asm_stream, "rdi");
                asm_stream.write_line("syscall");
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

    fn to_asm(&self, asm_stream: &mut AsmStream) {
        match self {
            Expr::I64(i64) => {
                asm_stream.write_line_string(format!("mov rax, {i64}"));
                STACK_IDX_TRACKER.push(asm_stream, "rax");
            }
            Expr::Ident(ident) => todo!(),
        }
    }
}
