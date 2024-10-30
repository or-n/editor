use crate::term::*;
use crate::term_text::{settings::*, token::Token};
use crate::token::*;

#[derive(Debug)]
pub enum Error {
    A(super::Error),
    Space1,
    F(super::Error),
    Dot,
    Space2,
    B(super::Error),
}

pub struct Term(pub BTerm);

impl Eat<Token, Error, ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        use Error::*;
        let (i, a) = BTerm::eat(i, Settings::all(false)).map_err(A)?;
        let i = Token::Whitespace(' ', 1).drop(i).map_err(|_| Space1)?;
        let (i, f) = BTerm::eat(i, Settings::all(false)).map_err(F)?;
        let i = Token::Special('.').drop(i).map_err(|_| Dot)?;
        let i = Token::Whitespace(' ', 1).drop(i).map_err(|_| Space2)?;
        let (i, b) = BTerm::eat(i, Settings::all(false)).map_err(B)?;
        Ok((i, Self(infixr(a, f, b))))
    }
}
