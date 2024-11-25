use crate::term::*;
use crate::term_from_text::{settings::*, token::Token};
use eat::token::*;

#[derive(Debug)]
pub enum Error {
    A(super::Error),
    Sep1,
    Dot,
    F(super::Error),
    Space2,
    B(super::Error),
}

pub struct Term(pub BTerm);

impl Eat<Token, Error, ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        use Error::*;
        let (i, a) = BTerm::eat(i, Settings::all(false)).map_err(A)?;
        let i = super::sep(i).map_err(|_| Sep1)?;
        let i = Token::Special('.').drop(i).map_err(|_| Dot)?;
        let (i, f) = BTerm::eat(i, Settings::all(false)).map_err(F)?;
        let i = Token::Whitespace(' ', 1).drop(i).map_err(|_| Space2)?;
        let (i, b) = BTerm::eat(
            i,
            Settings {
                infixr: false,
                ..Settings::all(true)
            },
        )
        .map_err(B)?;
        use crate::term::Term::*;
        let t = match *b {
            InfixL(b1, f2, b2) => infixl(infixl(a, f, b1), f2, b2),
            _ => infixl(a, f, b),
        };
        Ok((i, Self(t)))
    }
}
