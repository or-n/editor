use crate::term::*;
use crate::term_text::{settings::*, token::Token};
use crate::token::*;

#[derive(Debug)]
pub enum Error {
    A(super::Error),
    Sep,
    B(super::Error),
}

pub struct Term(pub BTerm);

impl Eat<Token, Error, Settings> for Term {
    fn eat(i: &[Token], data: Settings) -> Result<(&[Token], Self), Error> {
        use Error::*;
        let (i, a) = BTerm::eat(i, data).map_err(A)?;
        let i = Token::Whitespace(' ', 1).drop(i).map_err(|_| Sep)?;
        let (i, b) = BTerm::eat(i, data).map_err(B)?;
        Ok((i, Self(apply(a, b))))
    }
}
