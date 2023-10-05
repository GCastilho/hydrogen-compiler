use std::str::FromStr;

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

#[derive(Debug)]
pub enum TokenParseError {
    InvalidControlChar(char),
    InvalidToken(String),
}
