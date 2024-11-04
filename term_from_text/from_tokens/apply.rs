use crate::term::*;
use crate::term_from_text::{settings::*, token::Token};
use crate::util::token::*;

#[derive(Debug)]
pub enum Error {
    A(super::Error),
    Space,
    B(super::Error),
}

pub struct Term(pub BTerm);

impl Eat<Token, Error, ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        use Error::*;
        let (i, a) = BTerm::eat(i, Settings::all(false)).map_err(A)?;
        let i = Token::Whitespace(' ', 1).drop(i).map_err(|_| Space)?;
        let (i, b) = BTerm::eat(i, Settings::all(true)).map_err(B)?;
        use crate::term::Term::*;
        let t = match *b {
            Apply(b1, b2) => apply(apply(a, b1), b2),
            _ => apply(a, b),
        };
        Ok((i, Self(t)))
    }
}
