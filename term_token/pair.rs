use crate::term::*;
use crate::token::*;

use crate::term_text::{settings::*, token::Token};

#[derive(Debug)]
pub enum Error {
    L,
    A(super::Error),
    Comma,
    Space,
    B(super::Error),
    R,
}

pub struct Term(pub BTerm);

impl Eat<Token, Error, ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        use Error::*;
        use Token::*;
        let i = Special('(').drop(i).map_err(|_| L)?;
        let (i, a) = BTerm::eat(i, Settings::all(true)).map_err(A)?;
        let i = Special(',').drop(i).map_err(|_| Comma)?;
        let i = Whitespace(' ', 1).drop(i).map_err(|_| Space)?;
        let (i, b) = BTerm::eat(i, Settings::all(true)).map_err(B)?;
        let i = Special(')').drop(i).map_err(|_| R)?;
        Ok((i, Self(pair(a, b))))
    }
}
