use crate::term::*;
use eat::*;

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

impl Eat<&str, Error, ()> for Term {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), Error> {
        use Error::*;
        let text = '('.drop(text).map_err(|_| L)?;
        let (text, a) = BTerm::eat(text, Settings::all(true)).map_err(A)?;
        let text = ", ".drop(text).map_err(|_| Sep)?;
        let (text, b) = BTerm::eat(text, Settings::all(true)).map_err(B)?;
        let text = ')'.drop(text).map_err(|_| R)?;
        Ok((text, Self(pair(a, b))))
    }
}
