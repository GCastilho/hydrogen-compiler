use super::token::{Token, TokenParseError};
use std::{char, io, iter::Peekable};
use thiserror::Error;

pub struct TokenParser<I: Iterator<Item = io::Result<char>>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = io::Result<char>>> TokenParser<I> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator<Item = io::Result<char>>> Iterator for TokenParser<I> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next()? {
            Ok(char) => match Token::try_from(char) {
                Ok(token) => Some(Ok(token)),
                Err(_) => {
                    if char.is_whitespace() {
                        return self.next();
                    }
                    let mut buf = vec![char];
                    while self
                        .iter
                        .peek()
                        .is_some_and(|v| v.as_ref().is_ok_and(|c| c.is_alphanumeric()))
                    {
                        buf.push(self.iter.next()?.unwrap());
                    }
                    let word = buf.into_iter().collect::<String>();
                    match word.parse() {
                        Ok(token) => Some(Ok(token)),
                        Err(err) => Some(Err(err.into())),
                    }
                }
            },
            Err(err) => Some(Err(err.into())),
        }
    }
}

pub trait Tokenizer {
    fn tokens(self) -> TokenParser<Self>
    where
        Self: Sized,
        Self: Iterator<Item = io::Result<char>>;
}

impl<I: Iterator<Item = io::Result<char>>> Tokenizer for I {
    fn tokens(self) -> TokenParser<Self>
    where
        Self: Sized,
        Self: Iterator<Item = io::Result<char>>,
    {
        TokenParser::new(self)
    }
}

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Error parsing token: {0}")]
    Parse(#[from] TokenParseError),
}
