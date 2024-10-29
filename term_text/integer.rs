use crate::term::*;
use crate::text::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl<'a> Eat<'a, Error, ()> for Integer {
    fn eat(text: &'a str, _data: ()) -> Result<(&'a str, Self), Error> {
        if let Ok((text, n)) = isize::eat(text, ()) {
            return Ok((text, i(n)));
        }
        if let Ok(text) = '+'.drop(text) {
            return Ok((text, add()));
        }
        if let Ok(text) = '*'.drop(text) {
            return Ok((text, mul()));
        }
        Err(Error::Invalid)
    }
}
