use crate::term::*;
use crate::text::*;

use super::*;

#[derive(Debug)]
pub enum Error {
    A(TermError),
    Sep,
    B(TermError),
}

pub struct Term(pub BTerm);

impl<'a> Eat<'a, Error, Settings> for Term {
    fn eat(text: &'a str, data: Settings) -> Result<(&'a str, Self), Error> {
        use Error::*;
        let (text, a) = BTerm::eat(text, data).map_err(A)?;
        let text = ' '.drop(text).map_err(|_| Sep)?;
        let (text, b) = BTerm::eat(text, data).map_err(B)?;
        Ok((text, Term(apply(a, b))))
    }
}
