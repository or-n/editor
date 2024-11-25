use crate::term::*;
use eat::text::*;

use super::*;

#[derive(Debug)]
pub enum Error {
    A(TermError),
    Space,
    B(TermError),
}

pub struct Term(pub BTerm);

impl Eat<Error, ()> for Term {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), Error> {
        use Error::*;
        let (text, a) = BTerm::eat(
            text,
            Settings {
                apply: false,
                ..Settings::all(true)
            },
        )
        .map_err(A)?;
        let text = ' '.drop(text).map_err(|_| Space)?;
        let (text, b) = BTerm::eat(text, Settings::all(false)).map_err(B)?;
        Ok((text, Self(apply(a, b))))
    }
}
