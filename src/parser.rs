use super::token::Token;
use std::{io, iter::Peekable, mem};

pub struct TokenParser<I: Iterator<Item = io::Result<char>>> {
    iter: Peekable<I>,
    buf: Vec<char>,
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
        match self.iter.next()? {
            Ok(char) => match char {
                ';' => Some(Ok(Token::Semi)),
                ' ' => self.next(),
                char => {
                    self.buf.push(char);
                    if let Ok(next) = self.iter.peek()? {
                        if !matches!(next, ';' | ' ') {
                            return self.next();
                        }
                    }
                    let word = mem::take(&mut self.buf).into_iter().collect::<String>();
                    match word.as_str() {
                        "return" => Some(Ok(Token::Return)),
                        i64_literal if i64_literal.parse::<i64>().is_ok() => {
                            Some(Ok(Token::I64Literal(i64_literal.parse().unwrap())))
                        }
                        invalid_token => todo!(),
                    }
                }
            },
            Err(err) => Some(Err(err)),
        }
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
