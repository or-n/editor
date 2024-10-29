use crate::term::*;
use crate::text::*;

use super::*;

#[derive(Debug)]
pub enum Error {
    Name,
    A(TermError),
    Sep,
    B(TermError),
    NewLine,
}

pub struct Term(pub BTerm);

impl Eat<Error, ()> for Term {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), Error> {
        use Error::*;
        let (text, name) = name::Term::eat(text, ()).map_err(|_| Name)?;
        let text = ": ".drop(text).map_err(|_| Sep)?;
        let (text, a) = BTerm::eat(text, Settings::default()).map_err(A)?;
        let text = '\n'.drop(text).map_err(|_| NewLine)?;
        let (text, b) = BTerm::eat(text, Settings::default()).map_err(B)?;
        Ok((text, Self(r#let(name.0, a, b))))
    }
}
