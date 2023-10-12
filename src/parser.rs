use crate::{grammar::Statement, token::Token};
use std::{collections::HashMap, fs::File, io::Write, iter::Peekable, mem, sync::Mutex};
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

    #[error("Undeclared Identifier {0}")]
    IdentifierUndeclared(String),
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

    pub fn to_asm(&self, output: File) {
        let mut asm_stream = AsmStream { output };
        asm_stream.write_line("global _start\n\n_start:");
        self.0.iter().for_each(|s| s.to_asm(&mut asm_stream));
    }
}

pub trait TreeParser: Sized {
    fn try_from_iter<I: Iterator<Item = Token>>(
        iter: &mut Peekable<I>,
    ) -> Result<Self, AstParserError>;

    fn to_asm(&self, asm_stream: &mut AsmStream);
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

pub struct AsmStream {
    output: File,
}

impl AsmStream {
    fn write(&mut self, data: &[u8]) {
        self.output.write_all(data).expect("write buf failed");
    }

    fn writeln(&mut self, data: &[u8]) {
        self.write(data);
        self.write("\n".as_bytes());
    }

    // pub fn write_label(&mut self, label: &str) {
    //     self.writeln(label.as_bytes());
    // }

    pub fn write_line(&mut self, line: &str) {
        self.write("  ".as_bytes());
        self.writeln(line.as_bytes());
    }

    pub fn write_line_string(&mut self, line: String) {
        self.write_line(&line)
    }
}

struct StackMetadata {
    stack_location: usize,
}

pub struct StackVarIdxMap {
    stack_size: Mutex<usize>,
    ident_stack_pos: Mutex<HashMap<String, StackMetadata>>,
}

impl StackVarIdxMap {
    pub fn new() -> Self {
        Self {
            stack_size: Mutex::new(0),
            ident_stack_pos: Mutex::new(HashMap::new()),
        }
    }

    pub fn write_push(&self, asm_stream: &mut AsmStream, reg: &str) {
        *self.stack_size.lock().unwrap() += 1;
        asm_stream.write_line_string(format!("push {reg}"));
    }

    pub fn write_pop(&self, asm_stream: &mut AsmStream, reg: &str) {
        *self.stack_size.lock().unwrap() -= 1;
        asm_stream.write_line_string(format!("pop {reg}"));
    }

    pub fn insert_ident(&self, ident: &str) {
        self.ident_stack_pos.lock().unwrap().insert(
            ident.to_string(),
            StackMetadata {
                stack_location: *self.stack_size.lock().unwrap(),
            },
        );
    }

    pub fn get_stack_offeset(&self, ident: &str) -> Option<usize> {
        self.ident_stack_pos
            .lock()
            .unwrap()
            .get(ident)
            .map(|v| ((*self.stack_size.lock().unwrap() - v.stack_location) - 1) * 8)
    }
}
