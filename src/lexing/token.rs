use super::token_type::TokenType;
use std::fmt;

pub struct Token {
    pub token_type: TokenType,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "token object is not displayable")
    }
}


