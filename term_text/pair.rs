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

impl<'a> Eat<'a, Error, Settings> for Term {
    fn eat(text: &'a str, data: Settings) -> Result<(&'a str, Self), Error> {
        use Error::*;
        let text = '('.drop(text).map_err(|_| L)?;
        let (text, a) = BTerm::eat(text, data).map_err(A)?;
        let text = ", ".drop(text).map_err(|_| Sep)?;
        let (text, b) = BTerm::eat(text, data).map_err(B)?;
        let text = ')'.drop(text).map_err(|_| R)?;
        Ok((text, Term(pair(a, b))))
    }
}
