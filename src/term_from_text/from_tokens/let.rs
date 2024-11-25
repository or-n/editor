use crate::term::*;
use crate::term_from_text::{settings::*, token::Token};
use eat::*;

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

impl Eat<&[Token], Error, ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        use Error::*;
        let (i, name) = super::eat_name(i).ok_or(Name)?;
        let i = Token::Special(':').drop(i).map_err(|_| Colon)?;
        let i = Token::Whitespace(' ', 1).drop(i).map_err(|_| Space)?;
        let (i, a) = BTerm::eat(i, Settings::all(true)).map_err(A)?;
        let i = Token::Whitespace('\n', 1).drop(i).map_err(|_| NewLine)?;
        let (i, b) = BTerm::eat(i, Settings::all(true)).map_err(B)?;
        Ok((i, Self(r#let(name, a, b))))
    }
}
