use crate::term::*;
use crate::text::*;

use super::*;

#[derive(Debug)]
pub enum Error {
    L,
    A(TermError),
    Sep,
    B(TermError),
    R,
}

pub struct Term(pub BTerm);

impl<'a> Eat<'a, Error, ()> for Term {
    fn eat(text: &'a str, _data: ()) -> Result<(&'a str, Self), Error> {
        use Error::*;
        let text = '('.drop(text).map_err(|_| L)?;
        let (text, a) = BTerm::eat(text, Settings::default()).map_err(A)?;
        let text = ", ".drop(text).map_err(|_| Sep)?;
        let (text, b) = BTerm::eat(text, Settings::default()).map_err(B)?;
        let text = ')'.drop(text).map_err(|_| R)?;
        Ok((text, Term(pair(a, b))))
    }
}
