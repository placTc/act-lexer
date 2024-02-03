use super::bracket::Bracket;
use super::keyword::Keyword;
use super::literal::Literal;

pub enum TokenType {
    Literal(Literal),
    Keyword(Keyword),
    Bracket(Bracket),
}