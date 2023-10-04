use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Semi,
    Return,
    I64Literal(i64),
}

impl FromStr for Token {
    type Err = TokenParseError;

    fn from_str(word: &str) -> Result<Self, Self::Err> {
        let token = match word {
            ";" => Token::Semi,
            "return" => Token::Return,
            i64_literal if i64_literal.parse::<i64>().is_ok() => Token::I64Literal(i64_literal.parse().unwrap()),
            invalid_token => return Err(TokenParseError::InvalidToken(invalid_token.into())),
        };
        Ok(token)
    }
}

#[derive(Debug)]
pub enum TokenParseError {
    InvalidToken(String)
}
