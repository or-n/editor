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

impl Eat<Error, Settings> for Term {
    fn eat(text: &str, data: Settings) -> Result<(&str, Self), Error> {
        use Error::*;
        let (text, a) = BTerm::eat(text, data).map_err(A)?;
        let text = ' '.drop(text).map_err(|_| Sep)?;
        let (text, b) = BTerm::eat(text, data).map_err(B)?;
        Ok((text, Self(apply(a, b))))
    }
}
