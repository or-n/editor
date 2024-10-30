use crate::term_text::token::Token;
use crate::token::*;

pub struct Term(pub String);

impl Eat<Token, (), ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), ()> {
        let (i, token) = Token::eat(i, ())?;
        match token {
            Token::Name(name) => Ok((i, Term(name))),
            _ => Err(()),
        }
    }
}
