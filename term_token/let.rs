use crate::term::*;
use crate::term_text::{settings::*, token::Token};
use crate::token::*;

#[derive(Debug)]
pub enum Error {
    Name,
    Colon,
    Space,
    A(super::Error),
    NewLine,
    B(super::Error),
}

pub struct Term(pub BTerm);

impl Eat<Token, Error, ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        use Error::*;
        let (i, name) = super::name::Term::eat(i, ()).map_err(|_| Name)?;
        let i = Token::Special(':').drop(i).map_err(|_| Colon)?;
        let i = Token::Whitespace(' ', 1).drop(i).map_err(|_| Space)?;
        let (i, a) = BTerm::eat(i, Settings::all(true)).map_err(A)?;
        let i = Token::Whitespace('\n', 1).drop(i).map_err(|_| NewLine)?;
        let (i, b) = BTerm::eat(i, Settings::all(true)).map_err(B)?;
        Ok((i, Self(r#let(name.0, a, b))))
    }
}
