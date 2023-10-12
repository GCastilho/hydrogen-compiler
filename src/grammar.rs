use crate::{
    parser::{AsmStream, AstParserError, StackVarIdxMap, TokenIterator, TreeParser},
    token::Token,
};
use lazy_static::lazy_static;
use std::{collections::HashSet, iter::Peekable, sync::Mutex};

lazy_static! {
    static ref STACK_IDX_TRACKER: StackVarIdxMap = StackVarIdxMap::new();
    static ref DECLARED_VARIABLES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
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
                if DECLARED_VARIABLES.lock().unwrap().contains(&ident) {
                    return Err(AstParserError::IdentifierAlreadyUsed(ident));
                }
                DECLARED_VARIABLES.lock().unwrap().insert(ident.clone());
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
                STACK_IDX_TRACKER.write_pop(asm_stream, "rdi");
                asm_stream.write_line("syscall");
            }
            Statement::Let { ident, expr } => {
                STACK_IDX_TRACKER.insert_ident(ident);
                expr.to_asm(asm_stream);
            }
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
            Token::Ident(ident) => {
                if !DECLARED_VARIABLES.lock().unwrap().contains(&ident) {
                    return Err(AstParserError::IdentifierUndeclared(ident));
                }
                Self::Ident(ident)
            }
            _ => return Err(AstParserError::UnexpectedToken(token)),
        };
        Ok(expr)
    }

    fn to_asm(&self, asm_stream: &mut AsmStream) {
        match self {
            Expr::I64(i64) => {
                asm_stream.write_line_string(format!("mov rax, {i64}"));
                STACK_IDX_TRACKER.write_push(asm_stream, "rax");
            }
            Expr::Ident(ident) => {
                let stack_offset = STACK_IDX_TRACKER.get_stack_offeset(ident).unwrap();
                STACK_IDX_TRACKER.write_push(asm_stream, &format!("QWORD [rsp + {stack_offset}]"));
            }
        }
    }
}
