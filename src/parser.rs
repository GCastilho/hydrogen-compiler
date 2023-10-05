use super::token::Token;
use std::{io, iter::Peekable};

pub struct TokenParser<I: Iterator<Item = io::Result<char>>> {
    iter: Peekable<I>,
    buf: Vec<I>,
}

impl<I: Iterator<Item = io::Result<char>>> TokenParser<I> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            buf: Vec::new(),
        }
    }
}

impl<I: Iterator<Item = io::Result<char>>> Iterator for TokenParser<I> {
    type Item = io::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next();
        todo!()
    }
}

pub trait ParseTokenStream {
    fn tokens(self) -> TokenParser<Self>
    where
        Self: Sized,
        Self: Iterator<Item = io::Result<char>>;
}

impl<I: Iterator<Item = io::Result<char>>> ParseTokenStream for I {
    fn tokens(self) -> TokenParser<Self>
    where
        Self: Sized,
        Self: Iterator<Item = io::Result<char>>,
    {
        TokenParser::new(self)
    }
}
