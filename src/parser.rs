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

// TODO: Error deve ser um enum com io error ou TokenParseError
impl<I: Iterator<Item = io::Result<char>>> Iterator for TokenParser<I> {
    type Item = io::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next()? {
            Ok(char) => match Token::try_from(char) {
                Ok(token) => Some(Ok(token)),
                Err(_) => {
                    if matches!(char, ' ') {
                        return self.next();
                    }
                    self.buf.push(char);
                    if let Ok(next) = self.iter.peek()? {
                        if Token::try_from(*next).is_err() && !matches!(next, ' ') {
                            return self.next();
                        }
                    }
                    let word = mem::take(&mut self.buf).into_iter().collect::<String>();
                    match word.parse::<Token>() {
                        Ok(token) => Some(Ok(token)),
                        Err(_) => todo!(),
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
