use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub enum Token {
    Semi,
    Return,
    I64Literal(i64),
}

impl TryFrom<char> for Token {
    type Error = TokenParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let token = match value {
            ';' => Token::Semi,
            _ => return Err(TokenParseError::InvalidControlChar(value)),
        };
        Ok(token)
    }
}

impl FromStr for Token {
    type Err = TokenParseError;

    fn from_str(word: &str) -> Result<Self, Self::Err> {
        let token = match word {
            "return" => Token::Return,
            i64_literal if i64_literal.parse::<i64>().is_ok() => {
                Token::I64Literal(i64_literal.parse().unwrap())
            }
            invalid_token => return Err(TokenParseError::InvalidToken(invalid_token.into())),
        };
        Ok(token)
    }
}

#[derive(Debug, Error)]
pub enum TokenParseError {
    #[error("Invalid control character: {0}")]
    InvalidControlChar(char),

    #[error("Invalid token string: {0}")]
    InvalidToken(String),
}
